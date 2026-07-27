# Mathematics

Shared mathematical reference material underlying `docs/physics/`: vector/
quaternion conventions, the rigid-body collision math referenced from
`docs/physics/Collision.md`, and numerical integration schemes referenced
from `docs/physics/DeterministicSimulation.md`.

This directory is intentionally separate from `docs/physics/` so that
reusable math (e.g. "solving a 2D impulse system") isn't duplicated across
multiple phenomenon-specific physics docs — they should link here instead.

## Planned contents

- `VectorConventions.md` — shared vector/quaternion notation
- `ImpulseSolver.md` — the general impulse-based collision math used by
  both `docs/physics/Collision.md` and `docs/physics/Rail.md`
- `NumericalIntegration.md` — integration schemes considered for the
  coupled spin/friction system in `docs/physics/Spin.md` (RFC 0002)

Status: not yet written — populate alongside RFC 0001/0002 implementation.
