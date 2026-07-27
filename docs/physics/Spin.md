# Spin Dynamics

## State

Each ball carries a full 3D `AngularVelocity` vector (see
`docs/architecture/Components.md`), decomposed for physics purposes into:

- **Topspin/backspin component** — rotation about the horizontal axis
  perpendicular to travel direction
- **Side spin (English) component** — rotation about the vertical axis,
  see `English.md`
- **Swerve-inducing component** — combination effects when the cue is
  elevated, see `Swerve.md`

## Coupling with translation

Spin and translation are coupled through the contact-point-velocity
condition described in `Sliding.md`/`Rolling.md`: friction forces at the
cloth contact point are a function of relative sliding velocity, which
itself depends on both linear and angular velocity. CueForge integrates
these jointly each sub-step rather than treating spin decay as an
independent exponential decay — the latter is a common simplification in
simpler pool simulators but doesn't reproduce real draw/follow transition
behavior accurately.

## Spin transfer during collision

During ball-ball collision (`Collision.md`), a portion of angular
momentum can transfer between balls through the tangential (friction)
impulse at the contact point — this is the physical basis of `Throw.md`.

## Spin decay in the air (jump shots)

While `Airborne` (`Jump.md`), spin does not couple to cloth friction and
instead decays only through (negligible, currently unmodeled) air
resistance — see the "non-goals" note in `Jump.md`.
