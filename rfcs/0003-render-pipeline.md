# RFC 0003: Render Pipeline

- **Status**: Draft
- **Author(s)**: TBD
- **Created**: TBD
- **Related crates**: `crates/renderer`

## Summary

Defines the reference renderer's architecture: how it consumes
`docs/architecture/Events.md` and component state to draw a table, and
how it interpolates between fixed-timestep physics steps for smooth
presentation.

## Motivation

`crates/renderer` is the first consumer of the simulation core outside
the core itself, and its design will set the pattern that
`docs/architecture/PluginSystem.md` renderer plugins are expected to
follow.

## Guide-level explanation

The renderer subscribes to `Position`/`Velocity`/`BallState` component
changes and interpolates between the last two physics steps for smooth
motion at display refresh rate, independent of the fixed simulation
timestep (per `docs/architecture/Systems.md`'s note that presentation
interpolation lives outside the core step).

## Reference-level explanation

TBD — graphics API choice (e.g. wgpu), asset pipeline for table/ball
models, and the exact interpolation approach are open for RFC discussion.

## Drawbacks

TBD.

## Alternatives considered

TBD.

## Unresolved questions

- Graphics backend choice
- 2D reference renderer first vs. 3D from the start (leaning 2D-first for
  faster iteration on the physics validation loop, per `ROADMAP.md`
  milestone M3, but not yet decided)

## Documentation impact

`docs/architecture/Overview.md` layer diagram and
`docs/architecture/PluginSystem.md` renderer-plugin section should be
updated to reflect the concrete design once accepted.
