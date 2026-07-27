# English (Side Spin)

## Definition

"English" refers to side spin — rotation about the ball's vertical axis —
imparted by striking the cue ball left or right of its vertical
centerline. English is distinct from top/backspin (`Spin.md`) and
produces two related but distinct effects:

1. **Post-collision throw** on an object ball (`Throw.md`)
2. **Cushion-induced path deflection** off a rail (`Rail.md`)
3. **Squirt** at the moment of cue impact (`Squirt.md`)

## Model

English is represented as the vertical-axis component of a ball's
`AngularVelocity`. It does not, by itself, curve the cue ball's path
through open table (that requires swerve — elevated cue angle,
`Swerve.md`) — english on a level shot affects what happens *at contact*
(with a rail or another ball), not the free flight path.

## Interaction with rails

When a ball with side spin contacts a cushion, friction between the ball
and cushion during the (brief but non-instantaneous) contact alters the
rebound angle relative to the simple angle-of-incidence-equals-angle-of-
reflection case. This is modeled in `Rail.md` using a cushion friction
coefficient distinct from cloth friction.

## Interaction with object balls

See `Throw.md` for how side spin on the cue ball, combined with cut angle,
throws the object ball off the "natural" ghost-ball line.

## Common misconception this model corrects

English does **not** curve a ball in flight on a level table — that
requires an elevated cue and is a separate phenomenon (`Swerve.md`).
Conflating the two is a common simplification in arcade-style pool games
that CueForge explicitly avoids.
