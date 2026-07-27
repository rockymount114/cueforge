# Networking

## Why determinism enables this

Because the simulation core is fully deterministic given the same inputs
(`docs/physics/DeterministicSimulation.md`), CueForge can use
**lockstep-style netcode**: peers exchange `ShotInput` (and any seeded RNG
state) rather than continuous position updates, and each peer's local
simulation core produces identical results.

## What gets sent over the wire

- `ShotInput` (cue angle, force, contact point) — small, infrequent
- Periodic state checksums (hash of all `Position`/`Velocity`/`BallState`
  components) to detect desync early rather than silently diverging
- On desync detection: full authoritative state resync from host

## What does *not* get sent

- Per-tick position updates during a shot — each peer simulates the shot
  locally from the same `ShotInput`, since results are guaranteed
  identical.

## Open questions (candidates for RFC 0004)

- Host-authoritative vs. peer-to-peer with rollback
- How to handle floating-point differences across CPU architectures
  (candidate mitigation: fixed-point math in the physics core, or a
  strictly-specified floating-point mode — see
  `docs/physics/DeterministicSimulation.md`)
- Reconnection / spectator support

This document will be superseded in detail by `rfcs/0004-network-sync.md`
once accepted; until then it captures the reasoning for why lockstep is
the leading design.
