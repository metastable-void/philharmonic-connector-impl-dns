# Changelog

All notable changes to this crate are documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
this crate adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No unreleased changes.

## [0.1.0] - 2026-05-18

- Added the first substantive `dns_query` connector
  implementation backed by `mechanics-dns`.
- Added endpoint policy gates for RR type allowlists, zone
  allowlists, zone blocklists, both-list overlay-deny semantics,
  and per-call timeout selection.
- Added response normalization to presentation-form
  `{type, name, ttl, data}` records and tests for policy,
  timeout, and RCODE error mapping behavior.

## [0.0.0]

Name reservation on crates.io. No functional content yet.
