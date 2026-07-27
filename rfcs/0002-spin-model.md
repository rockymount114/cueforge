# RFC 0002: Spin Model

- **Status**: Accepted
- **Author(s)**: CueForge Contributors
- **Created**: 2026-07-27
- **Related crates**: `crates/spin`, `crates/physics`, `crates/collision`

## Summary

Defines the coupled spin/friction integration described in
`docs/physics/Spin.md`, `English.md`, `Throw.md`, `Squirt.md`, and
`Swerve.md`, and how `crates/spin` interacts with `crates/physics`'s
step function established in RFC 0001.

## Motivation

Spin is the single biggest driver of "does this feel like real pool"
across draw/follow, english, throw, and swerve. Getting the coupling
right once, in a shared crate, avoids each phenomenon being implemented
as an ad hoc special case (see the repeated design note across
`docs/physics/Throw.md` and `Swerve.md`: these should *emerge* from the
shared model, not be hand-coded separately).

## Guide-level explanation

`crates/spin` provides the coupled sliding-friction/angular-velocity
integration used by `crates/physics`'s step function during both
free-rolling motion and post-collision motion. `crates/collision` calls
into `crates/spin` when resolving the tangential (friction) component of
a ball-ball or ball-rail impulse (`docs/physics/Collision.md`,
`Rail.md`).

## Reference-level explanation

To be filled in during RFC discussion: exact integration scheme (see
open question below), and the precise contact-point-velocity formulation
referenced across `docs/physics/Sliding.md` and `Spin.md`.

## Drawbacks

Coupling spin and translation integration tightly is more complex than
treating spin decay as an independent exponential — but the simpler
model demonstrably fails to reproduce draw/follow transitions correctly,
per the design notes in `docs/physics/Sliding.md`.

## Alternatives considered

- Independent exponential spin decay (simpler, but physically inaccurate
  per real-world draw/follow behavior) — rejected.

## Unresolved questions

- Exact numerical integration scheme for the coupled system (analytical
  closed-form per sub-phase vs. numerical sub-stepping) — needs research
  before this RFC can move from Draft to Proposed.

## Documentation impact

`docs/physics/Spin.md`, `English.md`, `Throw.md`, `Squirt.md`,
`Swerve.md`, `Massé.md` should have their `TODO: verify` items resolved
alongside implementation.
