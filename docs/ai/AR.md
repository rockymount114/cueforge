# Augmented Reality (AR)

## Goal

Overlay CueForge's aim assistance, shot evaluation, and training feedback
directly onto a real table via AR (e.g. projected or headset-based
overlay of the ghost ball, suggested aim line, or predicted cue ball
path described in `GhostBall.md` and `PositionPrediction.md`).

## Relationship to other components

CueForge AR depends on:

- `ComputerVision.md` — to know real ball positions
- `GhostBall.md` / `ShotEvaluation.md` — to compute what to overlay
- A rendering/projection layer outside the scope of this repository (see
  below)

## Scope for this repository

As with `ComputerVision.md`, the core repository defines the data
(ghost-ball position, suggested aim line, predicted position) that an AR
front end would consume, via the same plugin/event interfaces used by any
other renderer (`docs/architecture/PluginSystem.md`). The AR-specific
projection/headset rendering code is expected to live in a separate
`CueForge AR` repository, per `README.md`'s project family table.

## Status

Exploratory / longer-term. Not yet scheduled on `ROADMAP.md` beyond being
noted as a post-M6 direction; no RFC yet drafted.
