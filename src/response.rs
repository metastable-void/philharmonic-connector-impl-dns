use mechanics_dns::DnsRecord;
use serde::Serialize;

/// Response body for `dns_query`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DnsQueryResponse {
    /// DNS records returned by the resolver.
    pub records: Vec<DnsRecordResponse>,
}

impl DnsQueryResponse {
    pub(crate) fn from_records(records: Vec<DnsRecord>) -> Self {
        Self {
            records: records.into_iter().map(DnsRecordResponse::from).collect(),
        }
    }
}

/// DNS record in presentation form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DnsRecordResponse {
    /// DNS RR type.
    #[serde(rename = "type")]
    pub record_type: String,
    /// Owner name in DNS presentation form.
    pub name: String,
    /// TTL in seconds.
    pub ttl: u32,
    /// RDATA in presentation form.
    pub data: String,
}

impl From<DnsRecord> for DnsRecordResponse {
    fn from(value: DnsRecord) -> Self {
        Self {
            record_type: value.record_type_name(),
            name: value.name,
            ttl: value.ttl,
            data: value.data,
        }
    }
}
