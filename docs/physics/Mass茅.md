# Massé Shots

## Definition

A massé shot uses a steeply elevated (often near-vertical) cue to impart
extreme spin, producing a sharply curving — sometimes near-right-angle —
cue ball path. It's the extreme end of the swerve spectrum (`Swerve.md`).

## Why it's modeled separately from Swerve.md

At steep cue angles, additional effects become significant that are
negligible for normal shots:

- Larger downward force component, affecting normal force against the
  cloth and therefore friction magnitude
- Increased likelihood of the ball briefly leaving the cloth surface
  (interacting with `Jump.md` considerations at extreme elevation)
- Larger sensitivity to tip-offset precision — small aiming errors produce
  large path differences, which matters for `docs/ai/ShotEvaluation.md`
  when assessing shot difficulty

## Model status

Massé shares its core integration approach with `Swerve.md` (coupled
sliding friction + spin), extended to account for the steeper-angle
normal-force effects above. Exact treatment of the normal-force
adjustment is `TODO: verify` against cited research — flagged for
resolution alongside RFC 0001/0002 implementation.

## Validation difficulty

Massé shots are notoriously hard to measure precisely even for real
players, so `Validation.md` treats massé validation as lower-confidence /
wider-tolerance than straighter, more repeatable shot types, and notes
this explicitly rather than implying false precision.
