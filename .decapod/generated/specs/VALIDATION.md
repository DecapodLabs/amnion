# Validation## Proof Surfaces

| Gate | Command | Evidence |
| --- | --- | --- |
| Formatting | `cargo fmt --check` | command output |
| Tests | `cargo test` | projection/consumer test output |
| Lints | `cargo clippy -- -D warnings` | lint output |
| Governance | `decapod validate` | validation receipt and epoch |## Contract proof

- Render representative Pincher events for active, blocked, failed, and
  handed-off runs while preserving ids and evidence references.
- Verify unknown events, stale producer state, missing custody, and validation
  failures remain visible and cannot become success.
- Verify human controls wait for authoritative Pincher/Decapod results.
- Confirm README/specs continue to describe Pincher as the loop owner and
  Decapod as governance authority.## Promotion Gates

- No promotion from a protected branch or outside a Decapod workspace.
- No UI-only state may satisfy an approval, proof, validation, or promotion
  gate.
- Any new host transport must have a version, migration path, consumer proof,
  and explicit rollback/removal behavior.

<!-- decapod:capability-overlay:persistent-state:start -->

## Persistent State Validation Overlay

### Migration Proof Command
- Configure `repo.migration_validation.command` and its arguments as the executable migration proof; file presence is not proof
- The configured command MUST define its working directory, timeout, expected exit code, and evidence output

### Migration Tests
- All migrations MUST have integration tests
- Rollback procedures MUST be tested
- Data integrity checks post-migration

### Persistence Integration Tests
- Repository abstraction tested against real database
- Transaction boundary tests
- Concurrency conflict tests
- Data integrity validation after recovery
<!-- decapod:capability-overlay:persistent-state:end -->

<!-- decapod:capability-overlay:public-api:start -->

## Public API Validation Overlay

### Contract Tests
- All public endpoints MUST have contract tests
- Request/response schema validation on every request
- Compatibility regression tests for each version

### Security Tests
- Authentication bypass tests
- Malformed input handling tests
- Rate limit enforcement tests
- Token expiry/revocation tests
<!-- decapod:capability-overlay:public-api:end -->

## Evidence Artifacts

Record projection fixtures, source event ids, authority references, and the
Decapod validation epoch. Do not retain secrets.

## Regression Guardrails

- Unknown or stale source data cannot render as authoritative success.
- UI-only state cannot satisfy approval, proof, or promotion gates.
- Cross-repository interface changes require Pincher consumer evidence.

## Current implementation boundary

The repository is currently documentation and governance scaffolding without a
checked-in TUI or Pincher adapter. The first implementation slice should build
the smallest projection contract before adding presentation breadth.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `cbb46fea4ed69a2e244419b99c731f321e0d22223264c74afc034828705c58f2`
- Significant implementation surfaces: `.github/` (1 files), `README.md/` (1 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->
