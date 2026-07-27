# Physics Documentation

This directory contains the phenomenon-by-phenomenon physics model for
CueForge, summarized at a high level in the root `PHYSICS.md`.

## Reading order

If you're new to the physics core, read in this order:

1. `CoordinateSystem.md` — units and conventions used everywhere else
2. `Collision.md` — the central ball-ball interaction
3. `Rolling.md` / `Sliding.md` — how a ball moves between collisions
4. `Spin.md` / `English.md` — spin state and side-spin effects
5. `Throw.md`, `Squirt.md`, `Swerve.md` — secondary effects that make the
   simulation feel like a real table rather than idealized billiard balls
6. `Massé.md`, `Jump.md` — advanced/trick shots
7. `Rail.md`, `Pocket.md`, `Cloth.md` — table interaction
8. `Calibration.md` — how constants are tied to real-world measurements
9. `DeterministicSimulation.md` — the constraints every phenomenon above
   must satisfy
10. `Validation.md` — how each phenomenon is tested against real data

## Contribution rule

Per `AGENTS.md`: no change to `crates/physics`, `crates/collision`, or
`crates/spin` without an accompanying test in `tests/physics/` and, where
relevant, an update to `Validation.md`. Every constant used in these docs
must be sourced or explicitly marked `TODO: verify`.
