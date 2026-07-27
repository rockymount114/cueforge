# GEMINI.md

> Guidance for Google Gemini (and Gemini-based coding agents) working in the
> **CueForge** repository — *The Blender of Cue Sports*.
>
> This file is part of a multi-agent documentation set. See also:
> `CLAUDE.md`, `AGENTS.md` (shared cross-agent rules), and `docs/contributing/`.
> If any instruction here conflicts with `AGENTS.md`, **`AGENTS.md` wins** —
> this file only adds Gemini-specific notes on top of the shared baseline.

---

## 1. What this project is

CueForge is an open source, professional-grade cue sports (pool / billiards /
snooker) simulation and training platform, built in Rust as a Cargo
workspace of 20+ crates. It is meant to be developed and maintained to the
same standard as projects like **Bevy**, **Tokio**, **Rapier**, and the
**Rust** compiler itself: RFC-driven, heavily documented, heavily tested,
deterministic where correctness matters, and friendly to long-term
community contribution.

Long-term product family (not all exist yet — check `ROADMAP.md`):

- **CueForge** — core simulation engine / platform (this repo)
- **CueForge Studio** — table & ball asset editor
- **CueForge Vision** — computer vision / real-table tracking
- **CueForge AR** — augmented reality overlay
- **AI Pool Academy** — training & coaching product built on top of CueForge

## 2. Prime directives for Gemini in this repo

1. **Never modify the physics engine (`crates/physics`, `crates/collision`,
   `crates/spin`) without accompanying deterministic tests.** Physics
   changes must include or update cases under
   `docs/physics/Validation.md` and `tests/physics/`.
2. **Determinism is a hard constraint.** Simulation code must not depend on
   wall-clock time, thread scheduling order, floating-point non-determinism
   across platforms, or unseeded randomness. If you introduce randomness,
   it must go through the project's seeded RNG utilities in
   `crates/common`.
3. **Dependency direction is one-way**, following the workspace graph:
   `physics → collision → spin → table → renderer → trainer → ai → ui`.
   Never add a dependency that points "backwards" up this chain. If a
   change seems to require it, stop and propose an RFC instead.
4. **Every public API needs doc comments and a matching Markdown page.**
   New modules under `crates/*` should be reflected in the relevant
   `docs/architecture/` or `docs/physics/` file in the same change, not as
   a follow-up.
5. **No silent behavior changes to gameplay rules** (`crates/rules`).
   Rule changes affecting scoring, fouls, or game variants require an RFC
   under `rfcs/` before implementation.
6. **Large or architectural changes go through the RFC process**
   (`rfcs/NNNN-title.md`), not directly through a PR. Examples: new crate,
   new physics model, network protocol change, renderer pipeline change,
   AI coaching model change.

## 3. Repository map (high level)

```
crates/        Rust workspace members (physics, collision, spin, table,
                cue, renderer, replay, ai, trainer, vision, networking,
                ui, rules, statistics, common)
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

Full structure and rationale: `docs/architecture/Overview.md`.

## 4. Coding standards

- Rust edition and MSRV are pinned in the workspace `Cargo.toml` — do not
  bump without an RFC.
- Format with `cargo fmt` and lint with `cargo clippy -- -D warnings`
  before proposing any change; CI (`fmt.yml`, `clippy.yml`) enforces this.
- Prefer explicit units and named types over bare `f32`/`f64` in physics
  code (e.g. `Meters`, `RadiansPerSecond`) — see
  `docs/physics/CoordinateSystem.md` for the conventions already in use.
- Public crate APIs should be `#[non_exhaustive]` where forward
  compatibility matters, per `docs/contributing/`.
- Tests: unit tests live next to code; deterministic physics regression
  cases live in `tests/physics/` and are cross-referenced from
  `docs/physics/Validation.md`.

## 5. Documentation standards

- All prose documentation is written in English, Markdown, using Mermaid
  for diagrams (`docs/diagrams/`) so it renders on GitHub without
  external tools.
- New physics phenomena get their own file under `docs/physics/`
  (following the existing pattern: `Collision.md`, `Spin.md`, `English.md`,
  `Throw.md`, `Squirt.md`, `Swerve.md`, `Massé.md`, `Jump.md`, `Rail.md`,
  `Pocket.md`, `Cloth.md`, `Calibration.md`, `DeterministicSimulation.md`,
  `Validation.md`) rather than being appended to an unrelated file.
- New AI/coaching features get a page under `docs/ai/` (e.g.
  `ShotEvaluation.md`, `GhostBall.md`, `PositionPrediction.md`,
  `SafetyAnalysis.md`).
- Every RFC follows the template in `rfcs/0000-template.md` (motivation,
  design, alternatives considered, unresolved questions).

## 6. What Gemini should do before making changes

1. Read `docs/architecture/Overview.md` and the relevant `docs/physics/*`
   or `docs/ai/*` page for the area being touched.
2. Check `rfcs/` for an existing or in-progress RFC covering the change.
   If none exists and the change is architectural, draft one instead of
   coding directly.
3. Check `ROADMAP.md` to confirm the change is in scope for the current
   milestone.
4. Run `cargo test --workspace` and, for physics/collision/spin crates,
   the determinism/validation suite described in
   `docs/physics/Validation.md`, before presenting a change as complete.

## 7. What Gemini must not do

- Do not invent or guess physics constants, table specifications, or rule
  variants — cite `docs/physics/` or ask for a source; place assumptions
  clearly as "TODO: verify" rather than presenting them as validated.
- Do not restructure the crate/workspace layout, rename public APIs, or
  change the Cargo workspace dependency graph without an RFC.
- Do not remove or weaken existing determinism guarantees, test coverage,
  or CI checks to make a change land more easily.
- Do not add new third-party dependencies to `crates/physics`,
  `crates/collision`, or `crates/spin` without discussing the reasoning
  in an RFC — these crates are meant to stay minimal and auditable.

## 8. CI expectations

Every change should be able to pass, unmodified:

```
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

corresponding to `.github/workflows/fmt.yml`, `clippy.yml`, and `ci.yml`.

## 9. Relationship to other agent docs

- `AGENTS.md` — shared rules for **all** coding agents (Gemini, Claude
  Code, Codex, Cursor, OpenHands, Aider, Continue, Cline). Treat it as the
  baseline.
- `CLAUDE.md` — Claude-specific notes; useful cross-reference if working
  alongside Claude-authored changes in the same PR history.
- This file (`GEMINI.md`) — Gemini-specific notes only; keep it short and
  avoid duplicating content that belongs in `AGENTS.md`.

---

*This file describes intended practice for a project at the planning
stage. As crates, RFCs, and docs are actually created, keep this file in
sync with reality rather than aspirational structure.*
