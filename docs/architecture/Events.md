# Events

## Purpose

Events are the mechanism by which everything outside the simulation core
(`rules`, `statistics`, `renderer`, `replay`, `ai`, `networking`) observes
what happened during a step, without depending on core internals.

## Event types (initial set)

| Event | Emitted when | Key fields |
|---|---|---|
| `BallBallCollision` | two balls make contact | ball IDs, contact point, impact speed |
| `RailContact` | a ball contacts a cushion | ball ID, rail ID, incoming/outgoing velocity |
| `PocketCapture` | a ball enters a pocket | ball ID, pocket ID, entry velocity |
| `MotionStateChanged` | a ball's state transitions | ball ID, old state, new state |
| `ShotStarted` | a cue impact is applied | shot parameters |
| `ShotEnded` | all balls reach `Stationary` | duration, final positions |

## Ordering guarantees

Events within a single tick are emitted in a canonical, deterministic
order: by simulation sub-step, then by ascending entity ID, then by
time-of-impact within the sub-step. Consumers must not assume any other
ordering (e.g. "insertion order of a hash map") — see
`docs/physics/DeterministicSimulation.md`.

## Consumers

- `crates/rules` — turns raw events (e.g. `PocketCapture` on the cue ball)
  into rule-level facts (a scratch foul).
- `crates/replay` — records the full event stream, keyed by tick, as the
  canonical replay format (`Serialization.md`).
- `crates/statistics` — aggregates events into shot/match statistics.
- `crates/ai` — uses `ShotEnded` state to evaluate position play.
- `crates/networking` — events (not raw component diffs) are what gets
  synchronized between peers in lockstep netcode, since they're small and
  their order is already canonical.

## Adding a new event type

New event types are additive and generally don't need an RFC, unless they
change what data flows *into* the physics core (see `DataFlow.md`) — in
which case follow the RFC process.
