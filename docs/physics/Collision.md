# Ball-Ball Collision

## Model

CueForge models ball-ball collisions as impulsive contact between two
rigid spheres, using an instantaneous impulse solution rather than a
soft-body/penalty-force approach, for both performance and determinism
(discrete impulse math is exactly reproducible; iterative penalty solvers
are more prone to platform-dependent floating-point divergence).

## Normal impulse

At contact, let `n` be the unit vector from ball A's center to ball B's
center. The relative velocity along `n` determines the normal impulse
`j`, using a coefficient of restitution `e`:

```
j = -(1 + e) * (v_rel · n) / (1/m_A + 1/m_B)
```

`e` for ball-ball contact is close to 1 (near-elastic) but not exactly 1;
see `Calibration.md` for the measured value CueForge uses.

## Tangential impulse & spin transfer

Real ball-ball collisions are not perfectly smooth — friction at the
contact point transfers some spin between balls and induces "throw" (see
`Throw.md`). CueForge models this with a tangential friction impulse
bounded by `μ_ball * j` (Coulomb friction), following the standard
approach used in published billiards physics research (Alciatore, "The
Illustrated Principles of Pool and Billiards" — cross-check in
`Calibration.md`).

## Order of operations within a tick

1. Detect all ball-ball contacts for this tick (narrow phase).
2. Sort candidate collisions by time-of-impact, tie-broken by ascending
   `BallId` pair (see `docs/architecture/DeterministicSimulation.md`
   → *actually* `DeterministicSimulation.md` in this directory).
3. Resolve collisions in that order, re-checking for newly-created
   contacts after each resolution within the same sub-step.

## What this model does not (yet) capture

- Ball deformation during contact (treated as instantaneous/rigid)
- Contact-duration-dependent effects at very high impact speeds (break
  shots) — flagged as a known simplification, tracked for validation in
  `Validation.md`.
