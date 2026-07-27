# Roadmap

> Living document. Milestones move as reality dictates — update this file
> whenever scope changes, don't let it silently go stale.

## Guiding milestone structure

Each milestone should leave the repository in a state where
`cargo test --workspace` passes and the relevant docs are up to date.
"Foundation" milestones are documentation- and scaffolding-heavy;
later milestones are implementation-heavy.

## M0 — Foundation (completed)

- [x] Repository scaffold: crates, docs, RFC process, CI templates
- [x] Agent documentation (`AGENTS.md`, `GEMINI.md`, `CLAUDE.md`)
- [x] `rfcs/0001-physics-engine.md` accepted
- [x] `crates/common` — shared types (units, seeded RNG, math helpers)
- [x] `crates/physics` — minimal deterministic ball-ball collision model
- [x] Physics validation harness (`tests/physics/`, `docs/physics/Validation.md`)

## M1 — Core Simulation (completed)

- [x] `crates/collision`, `crates/spin` implemented against RFC 0001/0002
- [x] `crates/table`, `crates/cue` — table geometry, cue input model
- [x] Rail/cushion response (`docs/physics/Rail.md`) implemented
- [x] Pocket capture (`docs/physics/Pocket.md`) implemented
- [x] Determinism test suite covers full-shot simulation, not just pairwise
      collisions

## M2 — Rules & Game Layer

- [ ] `crates/rules` — 8-ball, 9-ball, straight pool variants
- [ ] `crates/statistics` — shot/match statistics tracking
- [ ] `crates/replay` — deterministic replay recording & playback

## M3 — Presentation

- [ ] `crates/renderer` — reference renderer (not the final product
      renderer; validates the simulation visually)
- [ ] `crates/ui` — minimal desktop UI for manual testing

## M4 — Networking

- [ ] RFC 0004 (network sync) accepted
- [ ] `crates/networking` — deterministic lockstep or rollback netcode

## M5 — AI & Training

- [ ] RFC 0005 (AI coach) accepted
- [ ] `crates/ai`, `crates/trainer` — shot evaluation, ghost-ball
      visualization, position prediction, safety analysis

## M6 — Vision (longer-term)

- [ ] RFC 0006 (computer vision) accepted
- [ ] `crates/vision` — real-table ball tracking prototype
- [ ] CueForge AR exploration (separate repo, depends on CueForge core)

## Explicitly out of scope for now

- Mobile builds
- Cloth wear / humidity simulation
- Non-Western cue sports variants beyond an initial well-documented set
