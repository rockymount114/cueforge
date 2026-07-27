# Throw

## Definition

"Throw" is the deflection of an object ball's path away from the
idealized ghost-ball collision line, caused by friction at the ball-ball
contact point during collision — driven by the cue ball's spin state and
the cut angle of the shot.

## Two components

1. **Collision-induced throw** — tangential friction during the brief
   ball-ball contact (`Collision.md`) transfers some sideways force,
   most pronounced on thin cut shots with side spin.
2. **Spin-transfer throw** — english on the cue ball partially transfers
   to the object ball, slightly altering its path independent of the
   collision-friction effect above.

## Model

Both components fall out of the same tangential-impulse model described
in `Collision.md` — CueForge does not model throw as a separate
after-the-fact angular correction; it emerges from correctly modeling
Coulomb friction at the contact point during collision resolution. This
is a deliberate design choice: throw amount as a function of cut angle
and speed should be an *output* of the collision model, not a hand-tuned
lookup table, so it stays consistent with `Calibration.md` data across
the full range of cut angles rather than only at measured sample points.

## Validation target

Published throw-angle-vs-cut-angle curves (e.g. from Alciatore's research)
are the reference dataset used in `Validation.md` to confirm the emergent
throw behavior matches measured reality within documented tolerance.
