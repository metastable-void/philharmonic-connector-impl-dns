# philharmonic-connector-impl-dns

DNS-querying connector implementation for the Philharmonic
workflow orchestration system.

## Status

Implemented. `dns_query` performs single DNS lookups in the
`dns` realm through `mechanics-dns`, which owns the
system-stub-resolver setup and `/etc/resolv.conf` ENOENT
fallback behavior. Endpoint config carries optional
`allowed_types`, `allowlist_zones`, `blocklist_zones`, and
`default_timeout_ms`; request JSON carries `{name, type,
timeout_ms}`. Policy gates run before resolver I/O and responses
return presentation-form `{type, name, ttl, data}` records.

## License

Dual-licensed under `Apache-2.0 OR MPL-2.0`.

## Contributing

Developed as part of the
[Philharmonic workspace](https://github.com/metastable-void/philharmonic-workspace).
Workspace-wide development conventions — git workflow, script
wrappers, Rust code rules, versioning, terminology — live in the
meta-repo, authoritatively in its
[`CONTRIBUTING.md`](https://github.com/metastable-void/philharmonic-workspace/blob/main/CONTRIBUTING.md).
