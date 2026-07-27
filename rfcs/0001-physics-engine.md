# RFC 0001: Physics Engine Foundation

- **Status**: Accepted
- **Author(s)**: CueForge Contributors
- **Created**: 2026-07-27
- **Related crates**: `crates/physics`, `crates/common`

## Summary

Establishes the foundational architecture of `crates/physics`: the `World`
representation, the fixed-timestep step function, the deterministic
integration approach, and the initial ball-ball collision model.

## Motivation

Every other crate in the workspace depends, directly or indirectly, on
`crates/physics`. Its core data structures and determinism guarantees
need to be settled before `crates/collision`, `crates/spin`, or any
higher-layer crate can be implemented against a stable contract.

## Guide-level explanation

`crates/physics` exposes a `World` (per
`docs/architecture/Components.md`/`Resources.md`) and a
`step(world: &mut World, dt: FixedTimestep) -> Vec<Event>` function.
Consumers advance the simulation by calling `step` repeatedly; they never
mutate ball state directly except through the documented downward-flowing
input path (`docs/architecture/DataFlow.md`).

## Reference-level explanation

- Fixed timestep value: TBD (candidate: matches physics literature
  standard sub-stepping used for stiff contact resolution — needs
  research, see `docs/physics/Calibration.md`)
- Unit types: per `docs/physics/CoordinateSystem.md`
- Determinism constraints: per `docs/physics/DeterministicSimulation.md`
  — this RFC is the first place those constraints get enforced in code,
  including the canonical event-ordering rule.
- Initial scope: ball-ball collision (`docs/physics/Collision.md`) and
  straight-line rolling/sliding (`Rolling.md`, `Sliding.md`) only; rails,
  pockets, and spin-coupling land in follow-up RFCs (0002+) to keep this
  RFC reviewable.

## Drawbacks

Splitting physics into multiple sequential RFCs (this one, then spin,
then rails/pockets) means the engine isn't feature-complete after RFC
0001 alone — acceptable tradeoff for reviewability.

## Alternatives considered

- A single mega-RFC covering all of physics — rejected as unreviewable
  and likely to stall.
- Building physics as one monolithic crate rather than splitting into
  `physics`/`collision`/`spin` — rejected; the split gives clearer test
  boundaries per `docs/physics/Validation.md`.

## Unresolved questions

- Exact fixed timestep value
- Whether to use `f32` or `f64` internally (determinism across platforms
  needs to be verified either way — see
  `docs/physics/DeterministicSimulation.md`)

## Documentation impact

`docs/physics/CoordinateSystem.md`, `Collision.md`, `Rolling.md`,
`Sliding.md`, `DeterministicSimulation.md` should be finalized (their
`TODO: verify` items resolved) alongside implementation of this RFC.
