use philharmonic_connector_impl_api::ImplementationError;
use philharmonic_connector_impl_dns::{DnsConfig, DnsQueryRequest, zone_matches};

fn request(name: &str, record_type: &str) -> DnsQueryRequest {
    DnsQueryRequest {
        name: name.to_owned(),
        record_type: record_type.to_owned(),
        timeout_ms: None,
    }
}

#[test]
fn empty_policy_allows_standard_queries() {
    let config = DnsConfig::default().prepare().expect("config prepares");

    request("www.example.com", "A")
        .prepare(&config)
        .expect("A query allowed");
    request("example.com", "MX")
        .prepare(&config)
        .expect("MX query allowed");
}

#[test]
fn allowlist_zone_policy_uses_final_label_suffixes() {
    let config = DnsConfig {
        allowlist_zones: Some(vec!["example.com".to_owned()]),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");

    request("www.example.com", "A")
        .prepare(&config)
        .expect("child passes");
    request("EXAMPLE.com", "A")
        .prepare(&config)
        .expect("case-insensitive exact zone passes");
    request("bar.foo.example.com", "A")
        .prepare(&config)
        .expect("deep child passes");

    let denied = request("notexample.com", "A")
        .prepare(&config)
        .expect_err("substring match denied");
    assert!(matches!(denied, ImplementationError::InvalidRequest { .. }));
    assert!(denied.to_string().contains("not in allowlist"));
}

#[test]
fn blocklist_zone_policy_denies_zone_and_children() {
    let config = DnsConfig {
        blocklist_zones: Some(vec!["secret.example.com".to_owned()]),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");

    for name in ["secret.example.com", "child.secret.example.com"] {
        let denied = request(name, "A")
            .prepare(&config)
            .expect_err("blocklisted zone denied");
        assert!(denied.to_string().contains("blocklisted"));
    }

    request("public.example.com", "A")
        .prepare(&config)
        .expect("public child allowed");
}

#[test]
fn blocklist_overlays_allowlist() {
    let config = DnsConfig {
        allowlist_zones: Some(vec!["example.com".to_owned()]),
        blocklist_zones: Some(vec!["secret.example.com".to_owned()]),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");

    request("public.example.com", "A")
        .prepare(&config)
        .expect("allowlisted public name passes");

    let blocklisted = request("secret.example.com", "A")
        .prepare(&config)
        .expect_err("overlay blocklist wins");
    assert!(blocklisted.to_string().contains("blocklisted"));

    let outside = request("notexample.com", "A")
        .prepare(&config)
        .expect_err("outside allowlist denied");
    assert!(outside.to_string().contains("not in allowlist"));
}

#[test]
fn allowed_types_policy_accepts_only_configured_types() {
    let config = DnsConfig {
        allowed_types: Some(vec!["A".to_owned(), "AAAA".to_owned()]),
        ..DnsConfig::default()
    }
    .prepare()
    .expect("config prepares");

    request("example.com", "A")
        .prepare(&config)
        .expect("allowed A passes");

    let denied = request("example.com", "MX")
        .prepare(&config)
        .expect_err("MX denied");
    assert!(denied.to_string().contains("not in allowed_types"));
}

#[test]
fn empty_name_is_invalid_request() {
    let config = DnsConfig::default().prepare().expect("config prepares");
    let denied = request("", "A")
        .prepare(&config)
        .expect_err("empty name denied");

    assert!(denied.to_string().contains("name required"));
}

#[test]
fn unknown_record_type_is_invalid_request() {
    let config = DnsConfig::default().prepare().expect("config prepares");
    let denied = request("example.com", "NOT_A_TYPE")
        .prepare(&config)
        .expect_err("unknown type denied");

    assert!(denied.to_string().contains("unknown record type"));
}

#[test]
fn unknown_record_type_in_allowed_types_is_invalid_config() {
    let denied = DnsConfig {
        allowed_types: Some(vec!["NOT_A_TYPE".to_owned()]),
        ..DnsConfig::default()
    }
    .prepare()
    .expect_err("unknown type in config denied");

    assert!(matches!(denied, ImplementationError::InvalidConfig { .. }));
    assert!(denied.to_string().contains("allowed_types"));
}

#[test]
fn public_zone_match_helper_does_not_use_substring_matching() {
    assert!(zone_matches("foo.example.com", "example.com"));
    assert!(!zone_matches("notexample.com", "example.com"));
}
