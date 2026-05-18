use std::time::Duration;

use mechanics_dns::RecordType;

use crate::config::PreparedConfig;
use crate::error::{Error, Result};

/// Conservative default DNS query timeout.
pub const DEFAULT_TIMEOUT_MS: u64 = 5_000;
/// Lower clamp for caller-supplied timeouts.
pub const MIN_TIMEOUT_MS: u64 = 100;
/// Upper clamp for caller-supplied timeouts.
pub const MAX_TIMEOUT_MS: u64 = 60_000;

pub(crate) fn enforce(config: &PreparedConfig, name: &str, record_type: RecordType) -> Result<()> {
    if !config.allows_type(record_type) {
        return Err(Error::InvalidRequest(format!(
            "type '{}' not in allowed_types",
            record_type
        )));
    }

    if let Some(allowlist) = config.allowlist_zones()
        && !allowlist.iter().any(|zone| zone_matches(name, zone))
    {
        return Err(Error::InvalidRequest(format!(
            "zone '{name}' not in allowlist"
        )));
    }

    if let Some(blocklist) = config.blocklist_zones()
        && blocklist.iter().any(|zone| zone_matches(name, zone))
    {
        return Err(Error::InvalidRequest(format!("zone '{name}' blocklisted")));
    }

    Ok(())
}

pub(crate) fn select_timeout(config_default_ms: Option<u64>, request_ms: Option<u64>) -> Duration {
    Duration::from_millis(clamp_timeout_ms(
        request_ms
            .or(config_default_ms)
            .unwrap_or(DEFAULT_TIMEOUT_MS),
    ))
}

pub(crate) fn clamp_timeout_ms(timeout_ms: u64) -> u64 {
    timeout_ms.clamp(MIN_TIMEOUT_MS, MAX_TIMEOUT_MS)
}

/// Case-insensitive exact-final-label suffix match.
pub fn zone_matches(name: &str, zone: &str) -> bool {
    let name_labels = labels(name);
    let zone_labels = labels(zone);
    if zone_labels.is_empty() || zone_labels.len() > name_labels.len() {
        return false;
    }

    name_labels
        .iter()
        .rev()
        .zip(zone_labels.iter().rev())
        .all(|(name, zone)| name.eq_ignore_ascii_case(zone))
}

fn labels(value: &str) -> Vec<&str> {
    value
        .trim()
        .trim_end_matches('.')
        .split('.')
        .filter(|label| !label.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DnsConfig;
    use crate::request::DnsQueryRequest;

    #[test]
    fn zone_matching_is_suffix_based_and_case_insensitive() {
        assert!(zone_matches("example.com", "example.com"));
        assert!(zone_matches("foo.example.com", "example.com"));
        assert!(zone_matches("bar.foo.example.com", "example.com"));
        assert!(zone_matches("EXAMPLE.com", "example.com"));
        assert!(zone_matches("foo.example.com.", "example.com"));
        assert!(!zone_matches("notexample.com", "example.com"));
        assert!(!zone_matches("example.org", "example.com"));
    }

    #[test]
    fn request_timeout_overrides_config_default() {
        let config = DnsConfig {
            default_timeout_ms: Some(10_000),
            ..DnsConfig::default()
        }
        .prepare()
        .expect("config prepares");
        let request = DnsQueryRequest {
            name: "example.com".to_owned(),
            record_type: "A".to_owned(),
            timeout_ms: Some(3_000),
        }
        .prepare(&config)
        .expect("request prepares");

        assert_eq!(request.timeout, Duration::from_millis(3_000));
    }

    #[test]
    fn default_timeout_is_used_when_no_override_exists() {
        let config = DnsConfig::default().prepare().expect("config prepares");
        let request = DnsQueryRequest {
            name: "example.com".to_owned(),
            record_type: "A".to_owned(),
            timeout_ms: None,
        }
        .prepare(&config)
        .expect("request prepares");

        assert_eq!(request.timeout, Duration::from_millis(DEFAULT_TIMEOUT_MS));
    }

    #[test]
    fn timeout_values_are_clamped() {
        assert_eq!(clamp_timeout_ms(1), MIN_TIMEOUT_MS);
        assert_eq!(clamp_timeout_ms(5_000), 5_000);
        assert_eq!(clamp_timeout_ms(999_999), MAX_TIMEOUT_MS);
    }
}
