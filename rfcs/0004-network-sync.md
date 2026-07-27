# RFC 0004: Network Synchronization

- **Status**: Draft
- **Author(s)**: TBD
- **Created**: TBD
- **Related crates**: `crates/networking`

## Summary

Defines the netcode model for multiplayer CueForge, building on the
determinism guarantees in `docs/physics/DeterministicSimulation.md` and
the design sketch in `docs/architecture/Networking.md`.

## Motivation

Networking is the area most sensitive to any residual non-determinism in
the physics core — this RFC forces those guarantees to be tested under
real conditions (cross-platform peers) rather than assumed.

## Guide-level explanation

See `docs/architecture/Networking.md` for the current design sketch
(lockstep, `ShotInput` exchange, periodic state checksums). This RFC is
where that sketch becomes a concrete, implementable protocol.

## Reference-level explanation

TBD — open questions below must be resolved first.

## Drawbacks

Lockstep netcode is sensitive to any per-peer simulation divergence; a
single non-deterministic bug in the physics core becomes a networking bug
here, which raises the bar on `docs/physics/DeterministicSimulation.md`
compliance.

## Alternatives considered

- Server-authoritative continuous state streaming (simpler to reason
  about but much higher bandwidth, and gives up the "replay = input log"
  compactness described in `docs/architecture/Replay.md`) — currently
  disfavored but not formally rejected.

## Unresolved questions

- Host-authoritative vs. true peer-to-peer
- Cross-architecture floating-point determinism verification plan
- Reconnection and spectator support

## Documentation impact

`docs/architecture/Networking.md` should be rewritten from "design
sketch" to "implemented protocol" once this RFC is accepted.
