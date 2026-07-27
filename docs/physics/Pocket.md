# Pocket Capture

## Model

A ball is captured by a pocket when its position and velocity satisfy the
pocket's capture geometry — not simply "center of ball crosses pocket
center," since real pocket capture depends on approach angle and speed
(a ball can "rattle" and fail to drop, or lip out).

## Capture geometry

Each `PocketGeometry` (see `docs/architecture/Components.md`) defines:

- A capture point/region (pocket throat center and radius)
- Jaw geometry (the two points where rail cushions terminate at the
  pocket), which affects both direct capture and the "rattle" case where
  a ball contacts a jaw before either dropping or being deflected back
  onto the table

## Approach-angle dependency

Pocket capture is more forgiving for balls approaching near the pocket's
central axis and less forgiving for balls approaching at a sharp angle to
the pocket jaws (real "tight pocket" behavior). CueForge models this via
an effective capture width that narrows as approach angle deviates from
the pocket's central axis, rather than a fixed circular capture radius.

## Rattle / lip-out

A ball that contacts a jaw without enough energy/angle to complete
capture is deflected using the same rail-contact impulse model as
`Rail.md` (jaws are treated as short cushion segments), which is what
allows "rattling out" to emerge naturally rather than being a
special-cased outcome.

## Calibration

Pocket capture width/forgiveness is one of the most player-perceptible
tuning parameters in any pool simulation — validation against measured
real-table pocket behavior is prioritized in `Validation.md`, with
separate calibration profiles for "tight" (tournament-cut) vs. "loose"
(bar-table) pockets — see `Calibration.md`.
