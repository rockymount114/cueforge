# Rail / Cushion Response

## Model

When a ball contacts a cushion, CueForge resolves an impulse similar in
structure to ball-ball collision (`Collision.md`), but against a static
(infinite-mass) surface, with two cushion-specific parameters:

- **Cushion restitution** `e_cushion` — determines rebound speed relative
  to incoming speed along the normal
- **Cushion friction** `μ_cushion` — determines how much the rebound
  angle deviates from simple reflection, driven by the ball's side spin
  (`English.md`) and vertical spin at contact

## Cushion geometry

Cushions are modeled as a piecewise-linear (or spline, TBD) profile per
rail, since real cushions have a nose profile that contacts the ball
slightly above its equator — this affects both the effective contact
normal and induces a small vertical impulse component. See
`docs/architecture/Components.md` → `RailGeometry`.

## Spin-driven angle deviation

A ball with side spin rebounds off a cushion at an angle that deviates
from angle-of-incidence-equals-angle-of-reflection, in the direction
consistent with real-world "rail english" — this is the rail equivalent
of `Throw.md` and uses an analogous tangential-friction-impulse approach.

## Multi-rail contact

For balls approaching very close to a pocket jaw or a corner, contact
with two rail segments in quick succession must be resolved in
deterministic order — see `docs/physics/DeterministicSimulation.md` and
`docs/architecture/Systems.md` for the shared tie-breaking rule.

## Calibration

`e_cushion` and `μ_cushion` vary by table/cushion rubber type and are
calibration targets — see `Calibration.md`.
