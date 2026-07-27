# Calibration

## Purpose

This document is the index of every physical constant used across
`docs/physics/`, where it comes from, and what confidence level it
carries. Per `AGENTS.md`, no physics constant may be invented — every row
below must eventually cite a source or be marked `TODO: verify`.

## Constant registry

| Constant | Used in | Value | Source | Confidence |
|---|---|---|---|---|
| Ball radius (pool) | `CoordinateSystem.md` | 0.028575 m | WPA equipment spec | `TODO: verify` primary source |
| Ball mass (pool) | `CoordinateSystem.md` | 0.170–0.171 kg | WPA equipment spec | `TODO: verify` primary source |
| Ball-ball restitution `e` | `Collision.md` | ~0.93–0.98 (range) | published billiards physics research | `TODO: verify`, narrow range |
| Ball-ball friction `μ_ball` | `Collision.md`, `Throw.md` | `TODO: verify` | — | not yet sourced |
| Rolling friction `μ_roll` | `Rolling.md` | `TODO: verify` (cloth-dependent) | — | not yet sourced |
| Sliding friction `μ_slide` | `Sliding.md` | `TODO: verify` (cloth-dependent) | — | not yet sourced |
| Cushion restitution `e_cushion` | `Rail.md` | `TODO: verify` | — | not yet sourced |
| Cushion friction `μ_cushion` | `Rail.md` | `TODO: verify` | — | not yet sourced |

## Calibration methodology (planned)

1. Start from published, peer-reviewed or widely-cited billiards physics
   research (e.g. Alciatore's published work on pool physics) for initial
   values.
2. Where CueForge-specific validation is possible (recorded real shots,
   high-speed camera data, or community-contributed measurement), compare
   simulated outcomes against measured outcomes and adjust within the
   physically plausible range from step 1.
3. Record the final chosen value, its source, and its confidence level in
   the table above — a value should never move from `TODO: verify` to a
   real number without a corresponding source note in this file.

## Relationship to Validation.md

`Calibration.md` is about *where constants come from*. `Validation.md` is
about *how the resulting simulation is tested against reality*. A
calibrated constant that fails validation should be revisited here, not
silently patched only in test tolerances.
