use std::time::Duration;

use mechanics_dns::{RecordType, parse_record_type};
use philharmonic_connector_impl_api::ImplementationError;
use serde::Deserialize;

use crate::config::PreparedConfig;
use crate::error::{Error, Result};
use crate::policy;

/// Request body for the `dns_query` implementation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DnsQueryRequest {
    /// Domain name to query.
    pub name: String,
    /// DNS RR type string.
    #[serde(rename = "type")]
    pub record_type: String,
    /// Per-query timeout override in milliseconds.
    pub timeout_ms: Option<u64>,
}

impl DnsQueryRequest {
    /// Validate request shape and endpoint policy without performing DNS I/O.
    pub fn prepare(
        self,
        config: &PreparedConfig,
    ) -> std::result::Result<PreparedRequest, ImplementationError> {
        self.prepare_inner(config)
            .map_err(ImplementationError::from)
    }

    pub(crate) fn prepare_inner(self, config: &PreparedConfig) -> Result<PreparedRequest> {
        let name = normalize_name(self.name)?;
        let record_type = parse_record_type(&self.record_type).map_err(|_| {
            Error::InvalidRequest(format!("unknown record type '{}'", self.record_type))
        })?;

        policy::enforce(config, &name, record_type)?;
        let timeout = policy::select_timeout(config.default_timeout_ms(), self.timeout_ms);

        Ok(PreparedRequest {
            name,
            record_type,
            timeout,
        })
    }
}

/// Validated DNS query request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedRequest {
    /// Normalized query name.
    pub name: String,
    /// Parsed RR type.
    pub record_type: RecordType,
    /// Per-call timeout.
    pub timeout: Duration,
}

fn normalize_name(name: String) -> Result<String> {
    let normalized = name.trim().trim_end_matches('.').to_owned();
    if normalized.is_empty() {
        return Err(Error::InvalidRequest("name required".to_owned()));
    }
    Ok(normalized)
}
