# Computer Vision

## Goal

Track a real physical table (ball positions, and ideally cue stick
position/angle) from camera input, to drive CueForge's simulation core
with real-world state — the foundation for CueForge Vision and,
longer-term, CueForge AR.

## Scope for this repository

The CueForge core repository defines the **interface** vision systems
integrate through (an `Input` plugin producing `ShotInput` and/or ball
position updates, per `docs/architecture/PluginSystem.md`) but the vision
implementation itself (camera calibration, ball detection/tracking model,
cue tracking) is expected to live in a separate `CueForge Vision`
repository, per the project family described in `README.md`.

## Why this separation

Computer vision has a very different dependency footprint (camera SDKs,
ML inference runtimes) than the deterministic, minimal-dependency physics
core — keeping it out of the core repo protects `crates/physics`,
`crates/collision`, and `crates/spin` from dependency creep, consistent
with `AGENTS.md` §2.7 (no new dependencies in physics-critical crates
without an RFC).

## Interface requirements (for the eventual CueForge Vision integration)

- Ball position updates must be translated into the same coordinate frame
  as `docs/physics/CoordinateSystem.md`.
- Any detected state must go through the normal downward-flowing
  configuration/input path described in
  `docs/architecture/DataFlow.md` — vision does not get privileged direct
  access to mutate simulation state.

## Status

Exploratory; formal design gated on RFC 0006
(`rfcs/0006-computer-vision.md`), targeted at Roadmap milestone M6.
