# Serialization

## Formats in use

| Data | Format | Crate | Notes |
|---|---|---|---|
| Replay files | binary, versioned | `crates/replay` | see below for versioning policy |
| Table/ball asset definitions | RON or TOML (TBD) | `crates/table` | human-editable, used by CueForge Studio |
| Network wire format | binary, versioned | `crates/networking` | optimized for size over readability |
| Config files | TOML | `crates/common` | |

## Versioning policy

Every serialized format includes a version field. Breaking changes to a
format require:

1. Bumping the version field
2. A migration path (or an explicit, documented decision not to support
   migration, e.g. for early pre-1.0 formats)
3. A note in `CHANGELOG.md`

This is especially important for replay files (`Replay.md`) — a replay
recorded with an old CueForge version should either still play back
correctly or fail with a clear version-mismatch error, never silently
produce wrong results.

## Determinism note

Serialization/deserialization of `World` state must round-trip exactly
(bit-for-bit for anything that feeds back into the physics core) — see
`docs/physics/DeterministicSimulation.md`. This rules out, for example,
serializing floats through a lossy text format for any value that
re-enters the simulation.
