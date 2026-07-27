# Plugin System

## Goal

Let third parties (a new UI, a new AI model, CueForge Studio, CueForge
Vision) extend CueForge without forking the core, and without the core
depending on any of them.

## Extension points

| Plugin type | Hooks into | Example |
|---|---|---|
| Renderer plugin | subscribes to component state + events | a stylized 2D renderer, a VR renderer |
| Rules plugin | subscribes to events, exposes foul/score API | a new game variant beyond 8-ball/9-ball/straight pool |
| Trainer plugin | subscribes to `ShotEnded` events | a custom drill generator |
| Input plugin | produces `ShotInput` | a computer-vision-driven real cue tracker (CueForge Vision) |

## Design constraints

- Plugins interact with CueForge **only** through the `Event` stream and
  documented component/resource types (`Components.md`, `Resources.md`)
  — never through crate-internal APIs.
- Plugins must not be able to make the simulation core non-deterministic.
  Any plugin that wants to influence physics (rather than just observe
  it) must do so through the documented downward-flowing configuration
  path (`DataFlow.md`), not by mutating state directly mid-step.
- A plugin should be implementable as an external crate depending only on
  the public API of `crates/common` and whichever leaf crate it
  integrates with.

## Status

The plugin trait/interface itself is not yet designed in code — this is
expected to be proposed as an RFC once the core (`physics`, `collision`,
`spin`, `table`) stabilizes enough to know what a stable extension
surface looks like. Track in `ROADMAP.md` milestone M2+.
