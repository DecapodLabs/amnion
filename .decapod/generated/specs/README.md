# Amnion project specs

These living specs define Amnion as the human-facing terminal host for the
Pincher loop engine and Decapod governance plane.

- `INTENT.md` defines the quiet, confidence-oriented TUI outcome.
- `ARCHITECTURE.md` defines projection, control, and state ownership.
- `INTERFACES.md` defines the Pincher event/state consumer boundary.
- `SEMANTICS.md` defines display and human-action semantics.
- `OPERATIONS.md` defines host startup, refresh, and failure handling.
- `SECURITY.md` defines the projection trust boundary.
- `VALIDATION.md` defines host and contract proof surfaces.

Amnion must not run Pincher's loop or duplicate Decapod's authoritative state.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `cbb46fea4ed69a2e244419b99c731f321e0d22223264c74afc034828705c58f2`
- Significant implementation surfaces: `.github/` (1 files), `README.md/` (1 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->
