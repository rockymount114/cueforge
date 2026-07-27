# Data Flow

## The one-way dependency graph

```
physics → collision → spin → table → cue → rules → statistics
        → renderer → replay → trainer → ai → vision → networking → ui
```

(See `AGENTS.md` for the authoritative statement of this rule — this
document explains *why* and *how data actually moves*, not just the
constraint.)

A crate may depend on any crate to its left/below in the graph, never to
its right/above. This guarantees the simulation core can be built, tested,
and reasoned about with zero knowledge of rendering, AI, or networking.

## Downward flow: configuration & commands

- A `ShotInput` (cue angle, force, contact point) flows from `ui` down
  through `cue` into the physics core.
- Table configuration (rail positions, pocket sizes, cloth friction)
  flows from `table` down into `physics`/`collision` as read-only
  parameters, never the reverse.

## Upward flow: state & events

- Each simulation step, the core emits `Event`s (see `Events.md`):
  `BallBallCollision`, `RailContact`, `PocketCapture`,
  `MotionStateChanged`.
- `rules` subscribes to these events to detect fouls and scoring.
- `renderer` subscribes to `Position`/`Velocity` component changes to
  draw the current frame.
- `replay` subscribes to the full event stream to build a deterministic
  recording.
- `ai`/`trainer` subscribe to post-shot state (ball positions once the
  table is stationary) to evaluate the resulting position.

## Why this matters for AI agents editing this repo

If a change requires data to flow "backwards" (e.g. the renderer needs to
influence physics, or the AI needs to mutate collision resolution
directly), that is a signal the change needs an RFC — it likely means a
new event type or a new downward-flowing configuration parameter is
needed, not a new dependency edge.
