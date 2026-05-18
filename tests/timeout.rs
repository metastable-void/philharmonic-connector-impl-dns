use std::time::Duration;

use philharmonic_connector_impl_dns::{
    DEFAULT_TIMEOUT_MS, DnsConfig, DnsQueryRequest, MAX_TIMEOUT_MS, MIN_TIMEOUT_MS,
};

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
fn neither_timeout_uses_default() {
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
fn config_timeout_is_clamped() {
    let low = DnsConfig {
        default_timeout_ms: Some(1),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");
    assert_eq!(low.default_timeout_ms(), Some(MIN_TIMEOUT_MS));

    let high = DnsConfig {
        default_timeout_ms: Some(999_999),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");
    assert_eq!(high.default_timeout_ms(), Some(MAX_TIMEOUT_MS));
}

#[test]
fn request_timeout_is_clamped() {
    let config = DnsConfig::default().prepare().expect("config prepares");
    let request = DnsQueryRequest {
        name: "example.com".to_owned(),
        record_type: "A".to_owned(),
        timeout_ms: Some(1),
    }
    .prepare(&config)
    .expect("request prepares");

    assert_eq!(request.timeout, Duration::from_millis(MIN_TIMEOUT_MS));
}
