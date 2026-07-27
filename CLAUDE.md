# CLAUDE.md

> Guidance for Claude Code (and other Claude-based coding agents) working
> in the **CueForge** repository — *The Blender of Cue Sports*.
>
> This file only adds Claude-specific notes on top of the shared baseline
> in `AGENTS.md`. Read `AGENTS.md` first — it applies to Claude too. If
> anything here conflicts with `AGENTS.md`, **`AGENTS.md` wins**.

---

## 1. Project summary

CueForge is an open source, professional-grade cue sports (pool, snooker,
carom) simulation and training platform written in Rust, organized as a
Cargo workspace of 20+ crates, built to the engineering and documentation
standards of projects like Bevy, Tokio, Rapier, and rust-lang. See
`docs/architecture/Overview.md` before making non-trivial changes.

## 2. Non-negotiables (restated from AGENTS.md — do not skip)

1. No changes to `crates/physics`, `crates/collision`, or `crates/spin`
   without deterministic tests in `tests/physics/` and, where relevant, an
   update to `docs/physics/Validation.md`.
2. Simulation code must stay deterministic — no wall-clock time, no
   scheduling-order dependence, no unseeded randomness.
3. Dependency direction is one-way:
   `physics → collision → spin → table → renderer → trainer → ai → ui`.
4. Gameplay rule changes and architectural changes require an RFC in
   `rfcs/` before implementation.
5. Never loosen CI (tests, lints, determinism checks) just to land a
   change.
6. Never invent physics constants or rule details — cite a source or mark
   as `TODO: verify`.

## 3. How Claude should approach a task in this repo

- **Plan before editing.** For anything touching more than one crate, or
  anything in `crates/physics` / `crates/collision` / `crates/spin` /
  `crates/rules`, write out a short plan (files to touch, tests to
  add/update, docs to update) before making edits. Use extended thinking
  for physics math — deriving collision/spin formulas is exactly the kind
  of task worth reasoning through step by step rather than pattern
  matching to a plausible-looking formula.
- **Read before writing.** Check the relevant `docs/physics/*.md` or
  `docs/architecture/*.md` page and any existing RFC before touching
  physics or architecture code. Don't guess at conventions already
  documented.
- **Prefer small, reviewable diffs.** Scope each change to one crate or
  one concern, consistent with `AGENTS.md` §8. If a task naturally spans
  multiple crates (e.g. a new physics phenomenon needs both `physics` and
  a `docs/physics/` page), that's fine — but keep unrelated refactors out
  of the same change.
- **Show your reasoning for physics/math changes.** When deriving or
  modifying a physics formula (e.g. throw, squirt, swerve, spin transfer),
  include the derivation or a citation in the corresponding
  `docs/physics/*.md` file, not just the final code — this is what makes
  the physics auditable by human maintainers later.
- **Ask rather than assume when a request is ambiguous about scope**
  (e.g. "improve the collision model" could mean a bug fix, a new RFC, or
  a full model replacement) — pick the most conservative interpretation
  and state the assumption, per the project's RFC-first culture, rather
  than silently doing the largest possible version of the task.

## 4. Verification checklist before calling a task done

```
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

Plus, for physics/collision/spin work: run the determinism/validation
suite in `docs/physics/Validation.md` and reference the specific test
cases touched or added in the summary of the change.

## 5. Documentation duties

- Update `docs/architecture/*.md`, `docs/physics/*.md`, or `docs/ai/*.md`
  in the same change as the code, following the file-naming patterns
  already established (see `AGENTS.md` §7).
- If the change implements or partially implements an RFC, reference the
  RFC number in the change summary and, if the RFC's status changes
  (e.g. proposed → implemented), update its header accordingly.
- Keep Mermaid diagrams under `docs/diagrams/` in sync with any
  architecture change that affects data flow or system boundaries.

## 6. What to flag back to a human maintainer rather than deciding alone

- Any change that would violate the one-way crate dependency graph.
- Any change to gameplay rules, scoring, or fouls without an existing RFC.
- Any new third-party dependency in `crates/physics`, `crates/collision`,
  or `crates/spin`.
- Any situation where a physics constant, table specification, or rule
  detail can't be sourced from `docs/physics/` and would otherwise have
  to be guessed.
- Any request that would change the Rust edition/MSRV or restructure the
  workspace layout.

## 7. Relationship to other agent docs

- `AGENTS.md` — shared baseline for all agents; treat as authoritative.
- `GEMINI.md` — Gemini-specific notes; useful cross-reference when
  reviewing history that mixes agent-authored changes.
- Keep this file short — anything that applies to all agents belongs in
  `AGENTS.md`, not duplicated here.

---

*This file describes intended practice for a project at the planning
stage. Keep it in sync with the real repository structure as crates,
RFCs, and docs are actually created.*
