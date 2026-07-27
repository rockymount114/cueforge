# Safety Analysis

## Goal

Evaluate defensive ("safety") shot options — shots not primarily intended
to pocket a ball, but to leave the opponent in a difficult position — and
compare them against offensive options identified by
`ShotEvaluation.md`/`PatternRecognition.md`.

## What makes a good safety

- Object ball(s) end up difficult for the opponent to hit (blocked by
  other balls, near a rail, long thin cut only)
- Cue ball ends up difficult for the opponent's next shot
- Low risk of an unintended "gift" (e.g. accidentally leaving an easy
  shot, or worse, pocketing a ball for the opponent's benefit under some
  rule variants)

## Approach

Safety analysis reuses the same simulation-based evaluation approach as
`ShotEvaluation.md` and `PositionPrediction.md`, but scores outcomes by
"difficulty for the *next* player" rather than "make probability for the
*current* player" — largely the same machinery, different objective
function.

## Dependency on rules

Because what counts as a legal/beneficial safety depends on the active
game variant (e.g. a safety in 9-ball differs from straight pool),
`SafetyAnalysis.md` depends on `crates/rules` for variant-specific
scoring of "how good is this outcome for me," not just raw physics
outcomes.

## Status

Design-stage; depends on `ShotEvaluation.md`, `PositionPrediction.md`,
and a stable `crates/rules` API.
