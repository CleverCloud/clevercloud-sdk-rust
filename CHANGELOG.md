# Changelog

All notable changes to the `clevercloud-sdk` crate are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-06-04

### Changed
- Update the Redis add-on version from `8.6.1` to `8.8.0`. Clever Cloud upgraded the
  Redis version it offers shortly after the `1.0.0` release, so follow the catalog:
  rename the `redis::Version` variant `V8dot6dot1` (repr `861`) to `V8dot8dot0`
  (repr `880`). `7.2.4` is kept for backward compatibility with existing instances.

## [1.0.0] - 2026-06-04

First stable release of `clevercloud-sdk`. This release graduates the crate to a
semver-stable 1.x public API, bundling what would have been `0.16.0` together with the
major dependency migrations that motivate the `0.x` → `1.0.0` bump. Note that the public
surface re-exports `oauth10a` 3.x (`clevercloud_sdk::oauth10a`), so a future `oauth10a`
major version will require a corresponding major bump of this crate.

### Added
- Cover the add-on `Version` `FromStr`/`Display` round-trip with tests for every
  supported PostgreSQL, Redis, Elasticsearch and MySQL version, plus an unknown-input
  rejection check (guards against a variant added to `Display` but missing from the
  `FromStr` catch-all arm).

### Changed
- **BREAKING:** bump `schemars` 0.8 → 1.2. Third-party integrations are now versioned
  optional features, so the feature set changes: `chrono` → `chrono04`,
  `indexmap1` → `indexmap2`, `bytes` → `bytes1`, `url` → `url2` (`uuid1` unchanged),
  and `derive` is now explicit.
- **BREAKING:** migrate `oauth10a` 2.x → 3.0.0 (a substantial API rewrite with a
  generic `RestClient<X>` and nested `Result` returns). `lib.rs` now exposes a thin
  compatibility facade — a `ClientError` (retaining the historical `StatusCode`
  variant) and `Request`/`RestClient` traits that flatten the nested result back to
  `Result<T, ClientError>` — and re-exports the `oauth10a` module so
  `clevercloud_sdk::oauth10a::{ClientError, reqwest, ..}` keeps resolving.
- Update the hard-coded add-on database version enums to match what the Clever Cloud
  API currently offers: PostgreSQL adds 18 (default 17), Redis adds 8.6.1 (7.2.4 kept
  for backward compatibility), Elasticsearch adds 9, and the MySQL parse-error message
  now lists 8.4.
- Update the README for the current API and crate version.

### Fixed
- Gate the `log` imports in `v4/functions/mod.rs` and `v4/functions/deployments.rs`
  behind the `logging` feature so `--no-default-features` builds compile.
- Gate the six `#[tracing::instrument(skip_all)]` attributes on `Credentials`
  impls/constructors in `lib.rs` behind the `tracing` feature, restoring
  `--no-default-features` builds.

### Breaking changes since 0.15.0
- **`schemars` 0.8 → 1.2** — the `jsonschemas` feature now pulls schemars 1.x and the
  optional schema-integration feature names changed (`chrono04`, `indexmap2`, `bytes1`,
  `url2`); downstream consumers deriving `JsonSchema` against this crate's types upgrade
  with it.
- **`oauth10a` 2.x → 3.0.0** — leaks into the public API: the re-exported
  `clevercloud_sdk::oauth10a` is now the 3.x module, and the
  `ClientError` / `Request` / `RestClient` facade is the supported surface.
- **MSRV declared as 1.91.0** (`Cargo.toml` `rust-version`, `rust-toolchain`, CI matrix).
  Note: the library itself still compiles on 1.85.0; the declared bump is driven by the
  `cargo-tarpaulin` coverage tooling, which transitively requires rustc 1.91.
- **Edition 2024 / MSRV 1.85.0** baseline (carried in from 0.13.0) remains part of the
  1.0.0 contract.

## [0.15.0] - 2025-03-31
### Changed
- **BREAKING:** remove the `async_trait` dependency; the `Request` and `RestClient`
  trait methods now return `impl Future<...>` instead of being `async fn`, changing
  their public signatures.
- **BREAKING:** bump `oauth10a` to 2.1.1 and remove the `tokio` feature flag (which
  forwarded to `oauth10a/tokio`).

## [0.14.0] - 2025-03-24
### Added
- Add a `plans` field to `Provider`.
### Changed
- **BREAKING:** rework the plan API — move plan types from `v4::addon_provider::plan`
  to `v2::plan` and adjust `Provider` / add-on accessors accordingly.

## [0.13.6] - 2025-03-19
### Fixed
- Update add-on provider error messages to match the available variants.

## [0.13.5] - 2025-03-19
### Added
- Add Azimutt and Otoroshi variants to `AddonProviderId`.

## [0.13.4] - 2025-03-17
### Added
- Add Matomo and Cellar variants to `AddonProviderId`.
### Changed
- Update dependencies.

## [0.13.3] - 2025-03-12
### Added
- Add a Keycloak variant to `AddonProviderId`.

## [0.13.2] - 2025-03-12
### Added
- Add a Metabase variant to `AddonProviderId`.
### Changed
- Ensure consistent ordering of `AddonProviderId` variants.

## [0.13.1] - 2025-03-07
### Changed
- Point the oauthless endpoint at the production `api-bridge` host.
- Update dependencies.

## [0.13.0] - 2025-02-21
### Added
- Retrieve API credentials from the local `clever-tools` configuration.
### Changed
- **BREAKING:** move to Rust edition 2024 and raise MSRV to 1.85.0.
### Fixed
- Fix an inverted condition when setting up the endpoint on add-on unassignment.

## [0.12.0] - 2025-02-17
### Changed
- **BREAKING:** migrate from `oauth10a` 1.x to 2.x (`default-features = false`,
  `client` feature), drop the direct `hyper` dependency, and move `thiserror` to 2.x.
- **BREAKING:** raise MSRV to 1.84.1.
- Rename the `trace` feature to `tracing`.
- Rename the functions API path from `organisations` to `organizations`.
- Update dependencies and general maintenance.
### Fixed
- Correct the functions deployment status handling.
- Update the supported PostgreSQL version (issue #58).

## [0.11.1] - 2023-07-31
### Changed
- **BREAKING:** raise the minimum supported Rust version.
- Update dependencies.

## [0.11.0] - 2023-05-29
### Added
- Functions products API: introduce the `v4` functions product surface and expose it
  through the SDK.

## [0.10.13] - 2023-03-13
### Changed
- Bump dependencies: `hyper` 0.14.25, `serde` 1.0.155, `oauth10a` 1.3.18.

## [0.10.12] - 2023-03-09
### Added
- Redis add-on provider: support version 7.0.4.
- Elasticsearch add-on provider: support version 8.
### Deprecated
- Redis add-on provider: deprecate version 6.2.6.
- Elasticsearch add-on provider: deprecate version 6.

## [0.10.11] - 2023-03-07
### Changed
- Pin a `rust-toolchain` file to enforce the minimum supported Rust version in CI and
  examples.
- Update dependencies.

## [0.10.10] - 2022-11-21
### Changed
- Update dependencies.

## [0.10.9] - 2022-10-07
### Changed
- Update dependencies.

## [0.10.8] - 2022-09-09
### Changed
- Derive `Eq` on the public add-on and add-on-provider types in `v2::addon` and
  `v4::addon_provider`.
- Update dependencies.

## [0.10.7] - 2022-08-10
### Changed
- Update dependencies.

## [0.10.6] - 2022-07-18
### Changed
- Update dependencies.

## [0.10.5] - 2022-07-01
### Changed
- Bump the `oauth10a` dependency.

## [0.10.4] - 2022-06-20
### Changed
- Update dependencies.

## [0.10.3] - 2022-06-02
### Changed
- Update dependencies.

## [0.10.2] - 2022-05-27
### Changed
- Bump dependencies: `oauth10a` 1.3.7, `uuid` 1.1.0.

## [0.10.1] - 2022-05-18
### Changed
- **BREAKING:** raise the minimum supported Rust version to 1.60.0.
- Update dependencies.

## [0.10.0] - 2022-04-08
### Added
- Support for the Elasticsearch add-on provider.
### Changed
- **BREAKING:** rename public add-on option structs in `v2::addon` — `AddonOpts` → `Opts`
  and `CreateAddonOpts` → `CreateOpts`.
- Resolve add-on environment variables from the config-provider add-on.

## [0.9.0] - 2022-04-07
### Added
- Config-provider add-on support with environment-variable helper accessors.
### Changed
- Bump dependencies: `tracing` 0.1.32, `log` 0.4.16, `oauth10a` 1.3.4.

## [0.8.0] - 2022-03-30
### Added
- Support for the configuration (config-provider) add-on provider, including a
  hard-coded `CONFIG_PROVIDER` plan constant.
### Changed
- Bump dependencies: `async-trait` 0.1.53, `oauth10a` 1.3.3, `hyper` 0.14.18.
### Removed
- Drop support for Redis 6.0.10 from the Redis add-on provider.
- Drop support for PostgreSQL 9.6 from the PostgreSQL add-on provider.

## [0.7.2] - 2022-02-28
### Changed
- Bump `oauth10a` to 1.3.2.

## [0.7.1] - 2022-02-21
### Changed
- Update dependencies.

## [0.7.0] - 2022-02-11
### Added
- Add PostgreSQL version 14 and Redis version 6.2.6 to the supported add-on-provider
  versions.
### Changed
- Bump `oauth10a` to 1.3.0 and adopt its proxy-capable OAuth1.0a client, reworking how
  the client is constructed and authenticated across the public API.
- Update dependencies.

## [0.6.0] - 2022-02-03
### Added
- Add the ability to list available zones via a new `v4/products/zones` API, with
  helpers to query application and HDS zones.
### Changed
- **BREAKING:** migrate error handling across the public API to the `thiserror` crate,
  changing the error types returned by add-on, add-on-provider, `myself` and zones
  operations.

## [0.5.4] - 2022-02-01
### Changed
- Update dependencies.

## [0.5.3] - 2022-01-12
### Changed
- Update dependencies.

## [0.5.2] - 2021-12-13
### Added
- Add a command-line interface example (`cleverctl`) demonstrating usage of the SDK.
### Changed
- Update dependencies.

## [0.5.1] - 2021-11-22
### Changed
- Relicense the crate to MIT.
- Declare a minimum supported Rust version of 1.56 and add the repository link in
  `Cargo.toml`.
- Update dependencies.

## [0.5.0] - 2021-10-26
### Added
- Add Pulsar as a supported add-on.
### Changed
- **BREAKING:** make `AddonOpts.version` and `AddonOpts.encryption` optional
  (`String` → `Option<String>`, skipped when serializing `None`) and derive `Default`.

## [0.4.0] - 2021-10-25
### Changed
- **BREAKING:** migrate the crate to Rust edition 2021.

## [0.3.0] - 2021-10-21
### Added
- Add MySQL, Redis and MongoDB add-on providers to the v4 add-on-provider API.

## [0.2.0] - 2021-10-15
### Added
- Add a `tracing` cargo feature (built on the `trace` and `tokio` flags) to instrument
  client operations.
### Changed
- Stop tracking `Cargo.lock` for the library and refresh `.gitignore`.

## [0.1.1] - 2021-10-12
### Changed
- Rename the `telemetry` cargo feature to `metrics`.
- Bump `oauth10a` to 1.0.1.

## [0.1.0] - 2021-10-12
### Added
- Initial release of the `clevercloud-sdk` crate: a Rust client and data structures to
  interact with the Clever Cloud API. Ships the v2 API surface (add-on management,
  `myself` / self account) and the v4 add-on-provider surface (PostgreSQL provider with
  plans), plus project scaffolding (CI workflow, license, README, code of conduct).

[1.0.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.15.0...v1.0.0
[0.15.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.6...v0.14.0
[0.13.6]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.5...v0.13.6
[0.13.5]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.4...v0.13.5
[0.13.4]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.3...v0.13.4
[0.13.3]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.2...v0.13.3
[0.13.2]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.1...v0.13.2
[0.13.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.11.1...v0.12.0
[0.11.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.13...v0.11.0
[0.10.13]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.12...v0.10.13
[0.10.12]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.11...v0.10.12
[0.10.11]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.10...v0.10.11
[0.10.10]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.9...v0.10.10
[0.10.9]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.8...v0.10.9
[0.10.8]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.7...v0.10.8
[0.10.7]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.6...v0.10.7
[0.10.6]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.5...v0.10.6
[0.10.5]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.4...v0.10.5
[0.10.4]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.3...v0.10.4
[0.10.3]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.2...v0.10.3
[0.10.2]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.1...v0.10.2
[0.10.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/CleverCloud/clevercloud-sdk-rust/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/CleverCloud/clevercloud-sdk-rust/releases/tag/v0.1.0
