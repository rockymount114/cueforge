# Swerve (Curve)

## Definition

Swerve is the curving flight path of the cue ball caused by combining
side english with an elevated cue angle (as opposed to a level stroke).
It's distinct from both squirt (instantaneous deflection at impact) and
throw (deflection at ball-ball contact) — swerve happens continuously
during travel across the cloth.

## Physical cause

With an elevated cue, the impulse imparts both side spin and a downward
component. Early in the shot, the ball is sliding (not yet rolling) with
its spin axis tilted; sliding friction against the cloth (`Sliding.md`)
during this phase pushes the ball's path to curve before it transitions
to rolling.

## Model

Swerve emerges from the same coupled sliding-friction/spin integration
described in `Spin.md` and `Sliding.md`, given:

- A non-zero vertical component of initial `AngularVelocity` (from cue
  elevation)
- The standard sliding-to-rolling transition

As with `Throw.md`, CueForge does not hand-model swerve as a separate
curve function — it should emerge from correctly integrating the coupled
spin/friction equations already used elsewhere. This is a key validation
target in `Validation.md`: if swerve doesn't emerge correctly from the
shared model, that indicates a bug in the sliding/spin integration, not a
missing "swerve system."

## Massé relationship

Extreme cue elevation (near-vertical) produces the much more dramatic
curve of a massé shot — see `Massé.md`, which is swerve's more extreme
sibling with additional airborne/near-vertical considerations.
