# Sliding Friction

## When a ball is sliding

Immediately after a cue impact (or after certain collisions), a ball's
contact point is moving relative to the cloth — it is `Sliding`, not yet
rolling. Sliding friction is significantly higher than rolling friction
and is what causes a struck ball to transition from a "skidding" path to
a natural roll, and is central to draw/follow shot behavior.

## Model

Sliding friction acts opposite to the contact-point velocity direction
(not necessarily the ball's center-of-mass velocity direction, which is
what makes draw/follow physically distinct from pure top/bottom spin
intuition):

```
a_slide = -μ_slide * g   (applied opposite contact-point velocity)
```

Angular deceleration/acceleration is computed jointly so that the
contact-point velocity decays toward zero, at which point the ball
transitions to `Rolling` (`Rolling.md`).

## Why this matters for draw/follow

- **Draw**: cue tip strikes below center → backspin → contact-point
  velocity opposes travel direction more strongly → sliding friction
  decelerates translation while spin persists → ball can reverse
  direction after a collision.
- **Follow**: cue tip strikes above center → topspin → contact-point
  velocity aligns with travel direction → ball "follows through" a
  collision.

This is why `Sliding.md` and `Spin.md` must be modeled jointly rather than
as independent 1D effects — see `Spin.md` for the full coupled equations.

## Coefficient

`μ_slide` is cloth-dependent and typically several times larger than
`μ_roll`; measured range to be finalized in `Calibration.md`.
