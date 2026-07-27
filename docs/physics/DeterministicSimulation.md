# Deterministic Simulation

## Why determinism is a hard requirement

Determinism (same input → bit-identical output, across platforms and
runs) is what makes the following features possible without heavy
special-case engineering:

- Compact replay files (`docs/architecture/Replay.md`)
- Lockstep networking (`docs/architecture/Networking.md`)
- Physics regression testing (`Validation.md`)
- Reproducible bug reports

This is a **hard requirement**, restated from `AGENTS.md`: no change to
`crates/physics`, `crates/collision`, or `crates/spin` may introduce
non-determinism.

## Sources of non-determinism to avoid

| Risk | Mitigation |
|---|---|
| Unseeded randomness | All randomness goes through a seeded, versioned RNG in `crates/common` (`docs/architecture/Resources.md` → `RngState`) |
| Wall-clock time | Simulation runs on a fixed logical timestep, never `SystemTime`/`Instant` inside the core |
| Hash-map iteration order | Deterministic containers (`Vec`, `BTreeMap`, or explicitly sorted iteration) only, in any code path that affects simulation output or event ordering |
| Floating-point differences across platforms/architectures | Restrict core math to operations guaranteed reproducible under Rust's IEEE-754 semantics; avoid platform intrinsics or FMA-dependent paths that can differ across targets; document any remaining risk here as it's discovered |
| Ambiguous collision-resolution order | Canonical ordering rule below |

## Canonical ordering rule

When multiple simulation events are candidates within the same tick
(e.g. two ball-ball collisions), they are resolved in this order:

1. Ascending time-of-impact within the tick
2. Ties broken by ascending `(BallId, BallId)` pair (lexicographic)
3. Remaining ties (should not occur under 1–2, but guarded regardless)
   broken by event-type priority: ball-ball before ball-rail before
   pocket-capture

## Stationary threshold

A ball transitions `Rolling → Stationary` once speed falls below a fixed
epsilon (value to be finalized alongside RFC 0001), applied identically
regardless of platform, to avoid divergent "never quite stops" behavior.

## Testing determinism itself

`tests/physics/` includes tests that run the same shot input multiple
times (including across CI's different runner platforms) and assert
bit-identical output — not just "close enough" — since determinism is a
binary property, not a tolerance-based one.
