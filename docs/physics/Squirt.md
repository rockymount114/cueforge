# Squirt (Cue Ball Deflection)

## Definition

"Squirt" (also called "cue ball deflection") is the small deviation of
the cue ball's initial travel direction away from the cue stick's aimed
line, caused by applying side english. It happens at the moment of cue-tip
impact, before the ball even reaches the object ball.

## Physical cause

The cue tip contact point is offset from the cue ball's center of mass.
An off-center impulse imparts not just spin but a small sideways
component to the ball's initial linear velocity, because the impulse is
applied along the cue's axis, not through the ball's center.

## Model

Squirt angle is a function of:

- Tip offset (`TipOffset` component, see `docs/architecture/Components.md`)
- Cue shaft effective end-mass (a physical property of real cues — thinner
  low-deflection shafts squirt less; CueForge exposes this as a
  configurable cue property rather than a fixed constant)
- Impact force

## Relationship to squirt-compensation aiming

Experienced players aim slightly to compensate for expected squirt.
CueForge's AI coach (`docs/ai/ShotEvaluation.md`) accounts for this when
evaluating whether a suggested aim line will actually produce the
intended cue ball path.

## Status

`TODO: verify` — exact functional form (linear vs. tip-offset-squared) to
be finalized against cited research in `Calibration.md` before this is
implemented against RFC 0001/0002.
