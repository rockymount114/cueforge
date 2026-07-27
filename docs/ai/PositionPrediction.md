# Position Prediction

## Goal

Predict where the cue ball will come to rest after a given shot, so that
`ShotEvaluation.md` and `PatternRecognition.md` can reason about *next*
shot quality, not just whether the current shot is made.

## Approach

Because the underlying physics core is deterministic and fast
(`docs/physics/DeterministicSimulation.md`), position prediction is
implemented primarily by **running the simulation forward** for a
candidate shot, rather than a learned/approximate model — this keeps
predictions exactly consistent with what will actually happen when the
player takes the shot.

A learned/approximate model may later be added purely as a fast
pre-filter (to avoid full simulation of every candidate shot when
searching many options in `PatternRecognition.md`'s run-out search), but
final evaluation of any shot presented to the player must always come
from the real simulation, never the approximate pre-filter alone.

## Output

- Predicted cue ball final position (with uncertainty band, from the same
  perturbation approach described in `ShotEvaluation.md`)
- A "position quality" score relative to the next intended target ball —
  consumed by `PatternRecognition.md`'s run-out search
