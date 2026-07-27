# RFC 0005: AI Coach

- **Status**: Draft
- **Author(s)**: TBD
- **Created**: TBD
- **Related crates**: `crates/ai`, `crates/trainer`

## Summary

Defines the initial implementation scope for the AI Coach described in
`docs/ai/Coach.md`: shot evaluation and position prediction first, with
pattern recognition, safety analysis, and difficulty generation as
explicit follow-on phases.

## Motivation

`docs/ai/Coach.md` and its sub-documents describe the intended design,
but implementation needs a concrete, reviewable, phased plan rather than
attempting all six sub-components at once.

## Guide-level explanation

Phase 1 (this RFC's initial scope): `docs/ai/ShotEvaluation.md` and
`docs/ai/PositionPrediction.md`, built directly on the physics core via
repeated simulation (per the design principle in `docs/ai/Coach.md`:
physics-grounded, not black-box).

Later phases (tracked in `ROADMAP.md`, not blocked on this RFC being
reopened): pattern recognition, safety analysis, difficulty generation,
ghost-ball aim compensation.

## Reference-level explanation

TBD — depends on `crates/physics` and `crates/collision` being stable
enough to run repeated fast simulations for evaluation, which in turn
depends on RFC 0001/0002 being implemented.

## Drawbacks

Simulation-based evaluation (running many perturbed simulations per shot
candidate) may be too slow for real-time use without optimization —
flagged as a risk to validate early with benchmarks
(`docs/benchmarking/`).

## Alternatives considered

- Pure learned model trained on outcome data without a simulation
  backend — rejected as the primary approach per `docs/ai/Coach.md`'s
  design principle, though may be reconsidered as a fast pre-filter per
  `docs/ai/PositionPrediction.md`.

## Unresolved questions

- Performance budget for shot evaluation (how many perturbed simulations
  per candidate shot is affordable in real time)

## Documentation impact

`docs/ai/ShotEvaluation.md` and `docs/ai/PositionPrediction.md` should
have their "Status: design-stage" notes updated once implementation
begins.
