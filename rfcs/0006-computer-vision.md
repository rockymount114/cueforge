# RFC 0006: Computer Vision Integration

- **Status**: Draft
- **Author(s)**: TBD
- **Created**: TBD
- **Related crates**: none in this repo directly (defines the interface
  a separate CueForge Vision repository integrates through)

## Summary

Defines the `Input` plugin interface (per
`docs/architecture/PluginSystem.md`) that a computer-vision system uses to
feed real-table state into CueForge's simulation core, per the scope
boundary described in `docs/ai/ComputerVision.md`.

## Motivation

CueForge Vision is planned as a separate repository (see `README.md`'s
project family table), but the core repository needs to define a stable,
documented interface for it to integrate against before that work can
proceed independently.

## Guide-level explanation

A vision system produces `ShotInput` and/or direct ball-position
corrections, submitted through the same downward-flowing configuration
path as any other input source (`docs/architecture/DataFlow.md`) — it
receives no special/privileged access to simulation internals.

## Reference-level explanation

TBD — exact message format for position corrections (full state
replacement vs. incremental correction) is an open question.

## Drawbacks

Keeping vision fully external means CueForge core can't assume anything
about camera calibration or detection confidence — the interface needs to
be conservative and validated defensively (e.g. reject physically
impossible position updates) rather than trusting vision input blindly.

## Alternatives considered

- Building vision directly into this repository — rejected, per
  `docs/ai/ComputerVision.md`'s dependency-footprint rationale.

## Unresolved questions

- Format and confidence semantics of position-correction input
- How much latency/smoothing is handled in this repo's interface layer
  vs. in the external vision repo

## Documentation impact

`docs/ai/ComputerVision.md` and `docs/architecture/PluginSystem.md`
should be updated with the finalized interface once accepted.
