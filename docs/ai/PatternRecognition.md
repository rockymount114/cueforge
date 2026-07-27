# Pattern Recognition

## Goal

Identify higher-level structure in the current table layout that a human
coach would notice at a glance: clusters of balls that need to be broken
up, whether a run-out is likely feasible, which balls are "trouble balls"
near rails or clustered together, and roughly what order a strong player
would attempt to clear the table.

## Approach

- **Geometric clustering** — group balls that are close enough to
  interfere with each other's pocketing lines.
- **Run-out feasibility search** — using `ShotEvaluation.md` and
  `PositionPrediction.md` to search candidate shot orderings for a
  sequence that clears the table (a simplified planning search, not
  full game-tree search against an opponent).
- **Trouble-ball flagging** — balls whose only reasonable pocketing lines
  have low make-probability per `ShotEvaluation.md`.

## Relationship to Safety Analysis

Pattern recognition feeds `SafetyAnalysis.md`: if no good run-out order is
found, that's a signal a defensive shot may be the stronger option, which
safety analysis then evaluates concretely.

## Status

Design-stage. Depends on `ShotEvaluation.md` and `PositionPrediction.md`
being implemented first, since pattern recognition is built by composing
many individual shot evaluations rather than being its own independent
model.
