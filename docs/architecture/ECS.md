# Entity-Component-System (ECS) Model

## Summary

CueForge represents simulation state as **entities** (balls, the table,
the cue, pockets) with **components** (position, velocity, spin, radius,
mass) that are processed by **systems** (integration, collision
detection, collision response, rail response, pocket detection) each
simulation step.

## Why ECS for a physics-heavy simulation

- **Cache-friendly iteration.** Systems iterate over tightly packed
  component arrays (e.g. "all positions") rather than following pointers
  through an object graph — important given a full rack is 15+ balls
  each stepped many times per frame for accurate collision resolution.
- **Decoupling.** A rendering system can read `Position` and `Radius`
  without knowing anything about how `Spin` is computed. An AI system can
  read the whole `World` without being coupled to how physics integrates
  it.
- **Uniform replay/network story.** Because state is just component data,
  recording a replay or syncing state over a network is "serialize the
  components that changed," not "serialize special-cased game objects."

## Core entities

| Entity | Representative components |
|---|---|
| Ball | `Position`, `Velocity`, `AngularVelocity`, `Radius`, `Mass`, `BallState` (rolling/sliding/airborne/pocketed) |
| Table | `Geometry` (rails, pockets), `ClothFriction`, `CushionRestitution` |
| Cue | `TipPosition`, `Impact` (force, angle, offset — the shot input) |
| Pocket | `Position`, `CaptureRadius` |

## Core systems (run each step, in order)

1. Cue impact application (if a shot was just taken)
2. Integration (advance position/velocity/spin under motion state)
3. Ball-ball collision detection & response (`crates/collision`)
4. Ball-rail collision detection & response
5. Pocket capture detection
6. Motion-state transition (sliding → rolling → stationary)
7. Event emission

Exact ordering and rationale: `Systems.md`.

## What is *not* an ECS system

Rules enforcement (fouls, scoring), AI coaching, and rendering are **not**
part of the core ECS step — they consume the `Event` stream the core
emits (see `Events.md`) and live in their own crates, keeping the
simulation core free of game-variant-specific logic.
