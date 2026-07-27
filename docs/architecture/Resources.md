# Resources (Global / Shared State)

## What counts as a resource

Unlike components (per-entity data), resources are singleton state shared
across systems within a `World` — configuration and global simulation
parameters that aren't attached to any one ball or table element.

## Core resources

| Resource | Purpose |
|---|---|
| `SimulationConfig` | fixed timestep, gravity, global unit scale |
| `RngState` | seeded RNG state for any stochastic system (e.g. cloth micro-roughness, if modeled) — see `docs/physics/DeterministicSimulation.md` for why this must be seeded and versioned |
| `RulesetConfig` | which game variant is active (affects `crates/rules`, not the physics core itself) |
| `EventLog` | the append-only log of `Event`s emitted this shot, consumed by `replay` |

## Determinism implications

Any resource that influences physics output (`SimulationConfig`,
`RngState`) must be part of what gets recorded for a replay or
synchronized over the network — see `Replay.md` and `Networking.md`.
Resources that are purely presentational (e.g. a camera position) live in
`crates/renderer` and are explicitly excluded from the deterministic
core.
