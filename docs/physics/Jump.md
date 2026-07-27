# Jump Shots

## Definition

A jump shot causes the cue ball to leave the cloth surface (become
`Airborne`) to clear an obstructing ball, via a steep downward-angled
strike that causes the ball to compress against the cloth and rebound
upward, or (in some techniques) a direct elevated strike.

## Model

While `Airborne`:

- Standard projectile motion applies to `Position` under gravity.
- `AngularVelocity` is treated as constant (air resistance / spin decay
  in flight is not modeled — see Non-goals below).
- Cloth/rail friction systems (`Rolling.md`, `Sliding.md`, `Rail.md`) do
  not apply while `Airborne`.

Landing (return to cloth contact) transitions the ball back to `Sliding`
or `Rolling` depending on contact-point velocity at landing, using the
same logic as any other cloth-contact transition.

## Launch model

The initial launch velocity's vertical component, given a downward-angled
strike, is derived from the ball-cloth compression/rebound interaction —
this is the least first-principles part of the model (real jump-shot
launch is dominated by cue-tip/ball/cloth contact mechanics that are hard
to derive cleanly) and is expected to be **calibrated empirically** rather
than derived, with the calibration data and method documented in
`Calibration.md`.

## Non-goals

- Air resistance during flight (negligible at pool-ball speeds/typical
  jump heights; may be revisited if validation data shows otherwise)
- Ball-ball collisions while one ball is airborne (out of scope for the
  initial model; flagged as a known limitation)
