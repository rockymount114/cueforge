# Difficulty Generation

## Goal

Generate practice drills (ball layouts + shot objectives) at a target
difficulty level, for the `trainer` crate — e.g. "generate a medium-
difficulty cut-shot drill" or "generate a layout that requires a specific
position-play skill."

## Approach

1. Start from a shot archetype (straight-in, cut angle range, distance
   range, position-play requirement).
2. Generate candidate ball layouts satisfying the archetype's geometric
   constraints.
3. Use `ShotEvaluation.md` to estimate make-probability/difficulty for
   the generated layout, and iterate (reject/regenerate) until the
   estimated difficulty matches the requested target within tolerance.

## Why simulation-based generation, not templates

A fixed set of hand-authored drill templates would be far smaller in
variety and would drift out of sync with the actual physics model over
time (e.g. if `docs/physics/Calibration.md` values change, a
hand-authored "medium difficulty" drill might silently become easy or
hard). Generating and difficulty-scoring drills through the same
evaluation pipeline used elsewhere keeps difficulty labels accurate as
the engine evolves.

## Status

Design-stage; depends on `ShotEvaluation.md` being implemented and
validated (`docs/physics/Validation.md`) — difficulty labels are only as
trustworthy as the underlying make-probability estimates.
