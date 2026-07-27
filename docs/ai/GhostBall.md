# Ghost Ball Visualization

## What it is

The "ghost ball" is the classic pool-aiming aid: a rendered outline
showing where the cue ball's center needs to be at the moment of contact
with the object ball to send it toward the target pocket along the
desired line.

## Model

The ghost ball position is a straightforward geometric calculation: the
point on the object-ball's contact circle (radius = sum of both balls'
radii) that lies on the line from the object ball to the target pocket.

## Why this alone isn't sufficient (link to ShotEvaluation)

The naive ghost-ball line assumes a perfectly straight cue ball path to
contact, which is not true once `docs/physics/Squirt.md` (cue ball
deflection from english) is factored in. CueForge's ghost-ball
visualization is aim-compensated: for a chosen amount of english, the
displayed aim point accounts for expected squirt so the visualization
matches what the physics core will actually simulate, rather than
showing an idealized line the player then has to mentally correct.

## Rendering

Implemented as a `renderer` overlay, driven by data computed in `ai`
(specifically the aim-compensation logic) — see
`docs/architecture/PluginSystem.md` for how a renderer plugin consumes
this kind of computed overlay data.
