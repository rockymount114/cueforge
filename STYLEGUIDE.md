# Style Guide

## Rust

- Format with `cargo fmt` using the workspace's `rustfmt.toml` (default
  settings unless a crate has a documented reason to diverge).
- Lint clean with `cargo clippy --workspace -- -D warnings`.
- Follow the Rust API Guidelines (naming, `#[must_use]`, error types)
  unless a documented exception exists.
- Public crate APIs favor `#[non_exhaustive]` on structs/enums that may
  grow fields or variants.
- Physics/math code uses explicit unit-carrying types instead of bare
  `f32`/`f64` (e.g. `Meters`, `RadiansPerSecond`, `Newtons`) — see
  `docs/physics/CoordinateSystem.md`.
- No `unwrap()`/`expect()` in library crates outside of tests and cases
  proven unreachable by an accompanying comment; prefer `Result` with a
  crate-local error enum.
- No `unsafe` in `crates/physics`, `crates/collision`, or `crates/spin`
  without an RFC justifying it.

## Documentation

- English, Markdown, Mermaid for diagrams.
- Every doc page starts with a one-paragraph summary before any detail.
- Cite sources for physical constants and formulas; mark unverified
  values explicitly as `TODO: verify`.
- Prefer short declarative sentences over long qualified ones.

## Commit messages

- Present tense, imperative mood: "Add cushion restitution model", not
  "Added" or "Adding".
- Reference the RFC or issue number when applicable.

## Tests

- Unit tests live next to the code (`#[cfg(test)] mod tests`).
- Deterministic physics regression cases live in `tests/physics/`, named
  after the phenomenon and indexed from `docs/physics/Validation.md`.
- Benchmarks live in `benchmarks/`, using Criterion, and are referenced
  from `docs/benchmarking/`.
