# Rolling Friction

## When a ball is rolling

A ball is in the `Rolling` `BallState` once its contact-point velocity
(translation minus rotation-induced velocity at the cloth) reaches zero —
i.e. it's rolling without slipping. Rolling friction is much smaller than
sliding friction and acts to gradually decelerate the ball until it stops.

## Model

```
a_roll = -μ_roll * g
```

applied along the direction of travel, where `μ_roll` is the
rolling-friction coefficient (cloth-dependent, see `Calibration.md`) and
`g` is gravitational acceleration (9.81 m/s²).

Angular velocity is updated to stay consistent with the no-slip condition
(`v = ω × r`) as translational velocity decays.

## Transition to `Stationary`

A ball transitions from `Rolling` to `Stationary` once its speed falls
below a small epsilon threshold, defined in
`docs/physics/DeterministicSimulation.md` to avoid the ball asymptotically
"never quite stopping" in a way that differs across platforms.

## Cloth dependency

`μ_roll` varies with cloth type/condition and is one of the parameters
table presets can override — see `docs/physics/Cloth.md` and
`Calibration.md` for measured ranges across common cloth types (e.g.
simonis-style worsted cloth vs. napped cloth).
