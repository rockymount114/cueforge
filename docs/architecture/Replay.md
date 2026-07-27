# Replay

## Model

A replay is a recording of everything needed to deterministically
reproduce a shot (or a full match): the initial `World` state plus the
sequence of `ShotInput`s. Because the simulation core is deterministic,
CueForge does **not** need to record every frame of ball positions — it
replays by re-simulating from recorded inputs.

## Two replay modes

1. **Input replay** (default, small file size) — store initial state +
   `ShotInput` sequence; re-simulate on playback.
2. **Event replay** (for archival / cross-version playback) — additionally
   store the full `Event` log produced at recording time, so a replay
   remains viewable even if a later physics engine version would produce
   slightly different results for the same inputs.

## Why both

Input replay is the canonical, compact format and is what's used for
sharing shots. Event replay guards against the case where a physics
engine update (a bug fix, a new validated coefficient) would otherwise
silently make old input-replays play back differently — the recorded
event log lets old replays still be viewed "as they happened," while
input replays can be explicitly re-simulated against the new engine to
see the (documented) difference.

## File format

See `Serialization.md` for the concrete on-disk format.

## Relationship to networking

The `Event` log used for replay recording is the same stream consumed by
`crates/networking` for desync detection — see `Networking.md`.
