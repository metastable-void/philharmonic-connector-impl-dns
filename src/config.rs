use mechanics_dns::{RecordType, parse_record_type};
use philharmonic_connector_impl_api::ImplementationError;
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::policy;

/// Endpoint config for the `dns_query` implementation.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DnsConfig {
    /// Optional allowlist of DNS RR types.
    pub allowed_types: Option<Vec<String>>,
    /// Optional domain suffix allowlist.
    pub allowlist_zones: Option<Vec<String>>,
    /// Optional domain suffix blocklist.
    pub blocklist_zones: Option<Vec<String>>,
    /// Default timeout in milliseconds.
    pub default_timeout_ms: Option<u64>,
}

impl DnsConfig {
    /// Validate and normalize endpoint policy config.
    pub fn prepare(self) -> std::result::Result<PreparedConfig, ImplementationError> {
        self.prepare_inner().map_err(ImplementationError::from)
    }

    pub(crate) fn prepare_inner(self) -> Result<PreparedConfig> {
        let allowed_types = self
            .allowed_types
            .map(|types| {
                types
                    .into_iter()
                    .map(|record_type| {
                        parse_record_type(&record_type).map_err(|_| {
                            Error::InvalidConfig(format!(
                                "unknown record type '{record_type}' in allowed_types"
                            ))
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?;

        Ok(PreparedConfig {
            allowed_types,
            allowlist_zones: normalize_zones(self.allowlist_zones, "allowlist_zones")?,
            blocklist_zones: normalize_zones(self.blocklist_zones, "blocklist_zones")?,
            default_timeout_ms: self.default_timeout_ms.map(policy::clamp_timeout_ms),
        })
    }
}

/// Validated DNS endpoint policy config.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreparedConfig {
    allowed_types: Option<Vec<RecordType>>,
    allowlist_zones: Option<Vec<String>>,
    blocklist_zones: Option<Vec<String>>,
    default_timeout_ms: Option<u64>,
}

impl PreparedConfig {
    /// Allowed RR types, if configured.
    pub fn allowed_types(&self) -> Option<&[RecordType]> {
        self.allowed_types.as_deref()
    }

    /// Allowlisted DNS suffixes, if configured.
    pub fn allowlist_zones(&self) -> Option<&[String]> {
        self.allowlist_zones.as_deref()
    }

    /// Blocklisted DNS suffixes, if configured.
    pub fn blocklist_zones(&self) -> Option<&[String]> {
        self.blocklist_zones.as_deref()
    }

    /// Default timeout in milliseconds after clamping.
    pub fn default_timeout_ms(&self) -> Option<u64> {
        self.default_timeout_ms
    }

    pub(crate) fn allows_type(&self, record_type: RecordType) -> bool {
        self.allowed_types
            .as_ref()
            .is_none_or(|allowed| allowed.contains(&record_type))
    }
}

fn normalize_zones(zones: Option<Vec<String>>, field: &str) -> Result<Option<Vec<String>>> {
    zones
        .map(|zones| {
            zones
                .into_iter()
                .map(|zone| normalize_zone(zone, field))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()
}

fn normalize_zone(zone: String, field: &str) -> Result<String> {
    let normalized = zone.trim().trim_end_matches('.').to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(Error::InvalidConfig(format!(
            "{field} contains an empty zone"
        )));
    }
    Ok(normalized)
}
