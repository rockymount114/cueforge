# Shot Evaluation

## Goal

Given a table state (ball positions) and a candidate shot (target ball,
target pocket, cue ball contact point), estimate:

1. **Make probability** — likelihood the object ball is pocketed
2. **Resulting cue ball position** — where the cue ball ends up, and its
   quality for the next shot (see `PositionPrediction.md`)
3. **Risk** — what happens if the shot is missed (does it leave an easy
   shot for an opponent?)

## Approach

Because the physics core is deterministic and fast, shot evaluation can
run many simulated variations of a candidate shot (small perturbations in
aim, speed, spin — modeling human execution imprecision) and aggregate
outcomes, rather than relying purely on an idealized geometric
"ghost-ball line" calculation. The geometric ghost-ball line
(`GhostBall.md`) is the starting point/visualization; simulated
perturbation is what produces the probability estimate.

## Inputs from physics docs

- `docs/physics/Squirt.md`, `Throw.md` — needed so evaluated aim lines
  account for real deflection, not just idealized geometry
- `docs/physics/Collision.md` — needed for make-probability sensitivity
  to cut angle and speed

## Output consumed by

- `trainer` (drill feedback)
- `ui` (shot suggestion overlay)
- `SafetyAnalysis.md` (comparing offensive vs. defensive options)
