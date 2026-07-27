# Physics White Paper (Summary)

> This file is the front door to CueForge's physics documentation. The
> full, phenomenon-by-phenomenon treatment lives under `docs/physics/`.

CueForge models cue sports physics from first principles rather than
approximating with arcade-style heuristics. The goal is that a simulated
shot and a real shot, given the same initial conditions, diverge only by
the tolerances documented in `docs/physics/Validation.md`.

## Scope of the model

| Phenomenon | Doc |
|---|---|
| Coordinate system & units | `docs/physics/CoordinateSystem.md` |
| Ball-ball collision | `docs/physics/Collision.md` |
| Rolling friction | `docs/physics/Rolling.md` |
| Sliding friction | `docs/physics/Sliding.md` |
| Spin dynamics | `docs/physics/Spin.md` |
| English (side spin) | `docs/physics/English.md` |
| Throw | `docs/physics/Throw.md` |
| Squirt (cue ball deflection) | `docs/physics/Squirt.md` |
| Swerve (curve) | `docs/physics/Swerve.md` |
| Massé shots | `docs/physics/Massé.md` |
| Jump shots | `docs/physics/Jump.md` |
| Rail/cushion response | `docs/physics/Rail.md` |
| Pocket capture | `docs/physics/Pocket.md` |
| Cloth modeling | `docs/physics/Cloth.md` |
| Calibration to real tables | `docs/physics/Calibration.md` |
| Determinism guarantees | `docs/physics/DeterministicSimulation.md` |
| Validation methodology | `docs/physics/Validation.md` |

## Modeling philosophy

- **Analytical where possible, numerical where necessary.** Ball-ball
  collisions use closed-form impulse solutions; cushion and cloth
  interactions use validated empirical coefficients where a closed-form
  model doesn't exist or is too costly.
- **Every constant is sourced.** Coefficients of friction, restitution,
  and cushion response are either derived or cited from published
  research (see references in each physics doc) — never guessed.
- **Every phenomenon is independently testable.** Each doc under
  `docs/physics/` has a matching validation case in `tests/physics/`.

## Non-goals

CueForge's physics core does not attempt to model table manufacturing
tolerances, cloth wear over time, or humidity effects. These may become
optional, separately documented extensions later, but are out of scope
for the core engine.
