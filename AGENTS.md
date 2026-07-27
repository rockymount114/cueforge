# AGENTS.md

> Shared baseline rules for **all** AI coding agents working in the
> **CueForge** repository — *The Blender of Cue Sports*.
>
> This includes, but is not limited to: Gemini, Claude Code, OpenAI Codex,
> Cursor, OpenHands, Aider, Continue, and Cline.
>
> Agent-specific files (`GEMINI.md`, `CLAUDE.md`, etc.) may add extra notes
> for their own agent, but **must not contradict this file**. If a
> conflict exists, this file wins.

---

## 1. Project summary

CueForge is an open source, professional-grade cue sports (pool, snooker,
carom) simulation and training platform written in Rust, organized as a
Cargo workspace of 20+ crates. It targets the engineering and
documentation standards of projects such as Bevy, Tokio, Rapier, and
rust-lang itself: RFC-driven for major changes, heavily tested, heavily
documented, and deterministic wherever correctness depends on it.

Read `docs/architecture/Overview.md` before making any non-trivial change.

## 2. Hard rules (never break these)

1. **Never modify physics-critical crates** (`crates/physics`,
   `crates/collision`, `crates/spin`) **without adding or updating
   deterministic tests** in `tests/physics/` and, where relevant,
   `docs/physics/Validation.md`.
2. **Simulation code must be deterministic.** No dependence on wall-clock
   time, thread/task scheduling order, platform-specific floating-point
   behavior, or unseeded randomness. Randomness must go through the seeded
   RNG utilities in `crates/common`.
3. **Respect the one-way dependency graph:**
   `physics → collision → spin → table → renderer → trainer → ai → ui`.
   A crate may only depend on crates to its left. Do not add a "backwards"
   dependency; propose an RFC instead if one seems necessary.
4. **Gameplay rule changes require an RFC.** Anything affecting scoring,
   fouls, or game variants in `crates/rules` must be proposed in
   `rfcs/NNNN-title.md` before implementation.
5. **Architectural changes require an RFC**, including: new crate, new
   physics model, network protocol change, renderer pipeline change, save
   format change, or AI coaching model change.
6. **Do not weaken CI to make a change pass.** Do not delete, skip, or
   loosen tests, lints, or determinism checks to get a change merged.
7. **Do not invent physics constants, table specs, or rule variants.**
   Cite a source in `docs/physics/` or mark the value clearly as
   `TODO: verify` rather than presenting a guess as fact.

## 3. Required workflow before proposing a change

1. Read the relevant docs: `docs/architecture/Overview.md` plus whichever
   `docs/physics/*.md`, `docs/ai/*.md`, or `docs/architecture/*.md` file
   covers the area being touched.
2. Check `rfcs/` for an existing or in-progress RFC covering the change.
   Architectural changes without an RFC should get one drafted first,
   not coded directly.
3. Check `ROADMAP.md` to confirm the change is in scope for the current
   milestone. Out-of-scope work should be flagged, not silently done.
4. Make the change, keeping crate boundaries and the dependency graph
   intact.
5. Update or add documentation in the same change — not as a follow-up.
6. Run the full local verification (Section 5) before presenting the
   change as complete.

## 4. Repository map

```
crates/        Rust workspace members
docs/          architecture/, physics/, mathematics/, ai/, training/,
                roadmap/, api/, plugins/, contributing/, testing/,
                benchmarking/, diagrams/
rfcs/          numbered RFC proposals for major changes
examples/      runnable example programs
benchmarks/    Criterion-style benchmarks
tests/         integration & physics validation tests
scripts/       dev tooling scripts
tools/         internal tooling crates/binaries
.github/       CI workflows, issue templates, PR template, CODEOWNERS
```

Crate list: `physics`, `collision`, `spin`, `table`, `cue`, `renderer`,
`replay`, `ai`, `trainer`, `vision`, `networking`, `ui`, `rules`,
`statistics`, `common`.

## 5. Local verification (must pass before presenting a change as done)

```
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

For anything touching `physics`, `collision`, or `spin`, also run and
reference the determinism/validation suite described in
`docs/physics/Validation.md`.

## 6. Coding standards

- Follow the Rust edition/MSRV pinned in the workspace `Cargo.toml`; do
  not change it without an RFC.
- Use `cargo fmt` and `cargo clippy -- -D warnings` cleanly.
- Prefer explicit unit-carrying types over bare `f32`/`f64` in physics
  code (see `docs/physics/CoordinateSystem.md` for existing conventions).
- Public crate APIs should generally be `#[non_exhaustive]` where forward
  compatibility matters.
- Unit tests live next to the code they test; deterministic physics
  regression cases live in `tests/physics/` and are indexed from
  `docs/physics/Validation.md`.

## 7. Documentation standards

- All prose docs are written in English, in Markdown, using Mermaid for
  diagrams (`docs/diagrams/`) so they render natively on GitHub.
- New physics phenomena get their own file under `docs/physics/`
  following the existing naming pattern (`Collision.md`, `Spin.md`,
  `English.md`, `Throw.md`, `Squirt.md`, `Swerve.md`, `Massé.md`,
  `Jump.md`, `Rail.md`, `Pocket.md`, `Cloth.md`, `Calibration.md`,
  `DeterministicSimulation.md`, `Validation.md`) rather than being
  appended elsewhere.
- New AI/coaching features get a page under `docs/ai/` (e.g.
  `ShotEvaluation.md`, `GhostBall.md`, `PositionPrediction.md`,
  `SafetyAnalysis.md`).
- Every RFC follows `rfcs/0000-template.md` (motivation, design,
  alternatives considered, unresolved questions).

## 8. Commit / PR conventions

- Keep changes scoped to one crate or one concern where possible.
- PR descriptions should state: what changed, why, which docs/tests were
  updated, and which RFC (if any) it implements.
- Follow `.github/PULL_REQUEST_TEMPLATE.md` and applicable
  `.github/ISSUE_TEMPLATE/*` when opening issues or PRs.
- CODEOWNERS (`.github/CODEOWNERS`) determines required reviewers for
  physics-critical paths — do not bypass this.

## 9. Things agents commonly get wrong — watch for these

- Adding a dependency that violates the one-way crate graph "just to make
  it compile."
- "Fixing" a flaky physics test by loosening a tolerance instead of
  finding the source of non-determinism.
- Writing new gameplay rule logic directly in `crates/ui` or
  `crates/renderer` instead of `crates/rules`.
- Skipping documentation updates because "the code is self-explanatory."
- Treating this as a small game project rather than a long-lived platform
  — avoid one-off hacks that aren't extensible.

## 10. Per-agent files

- `GEMINI.md` — Gemini-specific notes.
- `CLAUDE.md` — Claude Code-specific notes.
- Additional per-agent files may be added following the same pattern;
  each should stay short and defer to this file for anything shared.

---

*This file describes intended practice for a project at the planning
stage. Keep it in sync with the real repository structure as crates,
RFCs, and docs are actually created.*
