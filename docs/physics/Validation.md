# Validation

## Purpose

This document describes how CueForge's physics model is tested against
reality, and indexes the validation cases in `tests/physics/`. Per
`AGENTS.md`, every phenomenon documented under `docs/physics/` should
have a corresponding entry here once implemented.

## Two kinds of tests

1. **Determinism tests** — same input produces bit-identical output,
   every time, on every supported platform. See
   `DeterministicSimulation.md`. These are pass/fail, not tolerance-based.
2. **Accuracy tests** — simulated outcome compared against real-world
   reference data (measured shots, published research figures) within a
   documented tolerance.

## Validation case index (planned; fill in as implemented)

| Phenomenon | Reference data source | Tolerance | Test file | Status |
|---|---|---|---|---|
| Ball-ball collision (head-on) | analytical solution (exact) | exact | `tests/physics/collision_head_on.rs` | implemented |
| Ball-ball collision (cut shots) | impulse / momentum conservation | exact | `tests/physics/collision_cut.rs` | implemented |
| Rolling deceleration | standard friction integration | exact | `tests/physics/rolling_decel.rs` | implemented |
| Draw/follow transition | spin-friction coupling | exact | `tests/physics/draw_follow.rs` | implemented |
| Determinism repeatability | 100-run comparison | bit-identical | `tests/physics/determinism.rs` | implemented |
| Swerve emergence | spin-driven acceleration | exact | `tests/physics/swerve.rs` | implemented |
| Rail rebound angle (no spin) | angle-of-incidence = angle-of-reflection | small tolerance | `tests/physics/rail_no_spin.rs` | implemented |
| Pocket capture / rattle | geometry threshold | exact | `tests/physics/pocket_capture.rs` | implemented |

## Tolerance philosophy

Tolerances should be set based on the precision of the reference data,
not loosened after the fact to make a failing test pass — see
`AGENTS.md` §2.6 ("do not weaken CI to make a change pass"). If a
tolerance seems wrong, the fix is to revisit `Calibration.md`, not the
test assertion.

## Confidence levels

Following `Massé.md`'s note on measurement difficulty: validation cases
for well-behaved, repeatable shots (straight rolls, head-on collisions)
should carry tight tolerances; validation for hard-to-measure shot types
(massé, jump) should carry explicitly wider, documented tolerances rather
than false precision.
