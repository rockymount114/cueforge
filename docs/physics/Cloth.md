# Cloth Modeling

## Scope

CueForge does not simulate cloth as a deformable surface (no cloth
mesh/particle simulation) — "cloth modeling" here refers to the
**frictional and damping properties** the cloth contributes to ball
motion, exposed as configurable table parameters.

## Parameters

| Parameter | Affects | Doc |
|---|---|---|
| `μ_roll` (rolling friction coefficient) | steady-state deceleration | `Rolling.md` |
| `μ_slide` (sliding friction coefficient) | draw/follow transition | `Sliding.md` |
| Cloth-cushion friction interaction | rail rebound | `Rail.md` |
| Nap direction bias (optional) | slight directional friction bias on napped cloth | not yet modeled — see Non-goals |

## Cloth presets

CueForge is expected to ship a small number of named cloth presets (e.g.
"tournament worsted," "standard napped") with measured or well-sourced
`μ_roll`/`μ_slide` values, documented in `Calibration.md`, rather than
one single global default — real cloth varies enough that a single
constant would misrepresent the sport.

## Non-goals (for now)

- Directional nap bias (napped cloth can play slightly differently with
  vs. against the nap) — real but secondary effect, deferred until core
  friction model is validated.
- Cloth wear/aging over a session — explicitly out of scope per
  `PHYSICS.md`.
- Humidity/temperature effects on friction — explicitly out of scope per
  `PHYSICS.md`.
