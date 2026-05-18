//! DNS query connector implementation for Philharmonic.
//!
//! `dns_query` implements the shared
//! [`philharmonic_connector_impl_api::Implementation`] trait for the
//! normalized DNS wire protocol described in the workspace connector
//! architecture docs. It delegates actual resolution to `mechanics-dns`
//! and keeps endpoint-local policy gates in front of the resolver call so
//! denied queries have no network side effect.

mod config;
mod error;
mod policy;
mod request;
mod response;

pub use crate::config::{DnsConfig, PreparedConfig};
pub use crate::policy::{DEFAULT_TIMEOUT_MS, MAX_TIMEOUT_MS, MIN_TIMEOUT_MS, zone_matches};
pub use crate::request::{DnsQueryRequest, PreparedRequest};
pub use crate::response::{DnsQueryResponse, DnsRecordResponse};
pub use philharmonic_connector_impl_api::{
    ConnectorCallContext, Implementation, ImplementationError, JsonValue, async_trait,
};

const NAME: &str = "dns_query";

/// `dns_query` connector implementation.
#[derive(Clone, Debug)]
pub struct DnsQuery {
    resolver: mechanics_dns::Resolver,
}

impl DnsQuery {
    /// Builds an implementation using the host's system resolver config.
    pub fn new() -> Result<Self, ImplementationError> {
        let resolver = mechanics_dns::Resolver::new()
            .map_err(|e| error::Error::Internal(e.to_string()))
            .map_err(ImplementationError::from)?;
        Ok(Self { resolver })
    }

    /// Builds an implementation with an externally constructed resolver.
    pub fn with_resolver(resolver: mechanics_dns::Resolver) -> Self {
        Self { resolver }
    }
}

#[async_trait]
impl Implementation for DnsQuery {
    fn name(&self) -> &str {
        NAME
    }

    async fn execute(
        &self,
        config: &JsonValue,
        request: &JsonValue,
        _ctx: &ConnectorCallContext,
    ) -> Result<JsonValue, ImplementationError> {
        let config: DnsConfig = serde_json::from_value(config.clone())
            .map_err(|e| error::Error::InvalidConfig(e.to_string()))
            .map_err(ImplementationError::from)?;
        let config = config.prepare_inner().map_err(ImplementationError::from)?;

        let request: DnsQueryRequest = serde_json::from_value(request.clone())
            .map_err(|e| error::Error::InvalidRequest(e.to_string()))
            .map_err(ImplementationError::from)?;
        let request = request
            .prepare_inner(&config)
            .map_err(ImplementationError::from)?;

        let records = tokio::time::timeout(
            request.timeout,
            self.resolver.query(&request.name, request.record_type),
        )
        .await
        .map_err(|_| ImplementationError::UpstreamTimeout)?
        .map_err(error::Error::from_resolver)
        .map_err(ImplementationError::from)?;

        let response = DnsQueryResponse::from_records(records);
        serde_json::to_value(response)
            .map_err(|e| error::Error::Internal(e.to_string()))
            .map_err(ImplementationError::from)
    }
}
