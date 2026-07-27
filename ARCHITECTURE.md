# Architecture

> High-level summary. Full detail lives under `docs/architecture/`.

## Guiding principles

1. **Determinism first.** The simulation core never depends on wall-clock
   time, thread scheduling, or unseeded randomness.
2. **One-way dependency graph.** Crates depend only "downward":
   `physics → collision → spin → table → renderer → trainer → ai → ui`.
3. **Small, focused crates.** Each crate does one thing and exposes a
   narrow, documented public API.
4. **ECS at the core.** Simulation state is modeled as entities and
   components, processed by systems, so the same world can be driven by a
   physics stepper, a replay player, or a network sync system
   interchangeably.
5. **Everything is a plugin above the core.** Rendering, training,
   AI coaching, and vision integrate through a plugin interface documented
   in `docs/architecture/PluginSystem.md`, not by reaching into engine
   internals.

## System layers

```
┌─────────────────────────────────────────────┐
│  ui / trainer / ai / vision / networking     │  application layer
├─────────────────────────────────────────────┤
│  renderer / replay                           │  presentation layer
├─────────────────────────────────────────────┤
│  table / cue / rules / statistics            │  game layer
├─────────────────────────────────────────────┤
│  spin / collision / physics / common         │  simulation core
└─────────────────────────────────────────────┘
```

Data flows down (configuration, commands) and up (state, events). See
`docs/architecture/DataFlow.md` and `docs/architecture/Events.md` for the
exact mechanisms.

## Where to go next

- [`docs/architecture/Overview.md`](docs/architecture/Overview.md) — full
  architecture narrative
- [`docs/architecture/ECS.md`](docs/architecture/ECS.md) — entity/component/
  system model
- [`docs/architecture/PluginSystem.md`](docs/architecture/PluginSystem.md) —
  how to extend CueForge without forking it
