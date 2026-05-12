# philharmonic-connector-impl-dns

DNS-querying connector implementation for the Philharmonic
workflow orchestration system.

## Status

**Not yet implemented.** This crate is reserved as part of the
[Philharmonic](https://github.com/metastable-void/philharmonic-workspace)
crate family and will provide arbitrary DNS lookup as a connector
implementation, using the host system's stub resolver
(`/etc/resolv.conf`) via
[`hickory-resolver`](https://crates.io/crates/hickory-resolver)
in system-config mode.

The implementation will expose a `dns_query` capability — single
DNS lookups parameterised by `{name, type, timeout_ms}`, returning
an array of resource records — with endpoint-config policy gates
for RR-type allowlisting and zone-level allowlist / blocklist
controls. `IN` class only; no custom recursive resolver; no
caching beyond what the host OS provides.

Implementation is scheduled for Phase 7 Tier 2. See
[`ROADMAP.md` §3.B / D19](https://github.com/metastable-void/philharmonic-workspace/blob/main/docs/ROADMAP.md)
and the
[connector-architecture §DNS spec](https://github.com/metastable-void/philharmonic-workspace/blob/main/docs/design/08-connector-architecture.md#dns)
for the timeline and wire shape.

## This is not name squatting

This name reservation is part of an active, shipped project.
The parent workspace has 21+ published crates on crates.io,
a full API server, end-to-end integration tests, and a
working deployment pipeline. This crate is in the
implementation queue — not a speculative hold.

If you believe this name conflicts with your project, please
open an issue at the
[workspace repository](https://github.com/metastable-void/philharmonic-workspace/issues).

## License

Dual-licensed under `Apache-2.0 OR MPL-2.0`.

## Contributing

Developed as part of the
[Philharmonic workspace](https://github.com/metastable-void/philharmonic-workspace).
Workspace-wide development conventions — git workflow, script
wrappers, Rust code rules, versioning, terminology — live in the
meta-repo, authoritatively in its
[`CONTRIBUTING.md`](https://github.com/metastable-void/philharmonic-workspace/blob/main/CONTRIBUTING.md).

SPDX-License-Identifier: Apache-2.0 OR MPL-2.0
