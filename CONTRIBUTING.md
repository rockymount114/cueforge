# Contributing to CueForge

Thank you for considering contributing. CueForge is built to the standard
of long-lived, community-governed open source projects — please read this
file before opening a PR.

## Before you start

1. **Read `docs/architecture/Overview.md`** and the doc page relevant to
   the area you're touching.
2. **Check `rfcs/`** for an existing or in-progress proposal covering your
   change. Architectural changes need an RFC *before* a PR with code.
3. **Check `ROADMAP.md`** to see whether the change is in scope for the
   current milestone.

## What needs an RFC

- New crates
- New physics models or changes to existing physics models
- Gameplay rule changes (scoring, fouls, variants)
- Network protocol changes
- Renderer pipeline changes
- AI coaching model changes

Open small bug fixes, doc fixes, and additive tests do **not** need an RFC.

## Development workflow

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

All three must pass before a PR is opened. For changes to
`crates/physics`, `crates/collision`, or `crates/spin`, also run the
determinism/validation suite described in `docs/physics/Validation.md`.

## Coding standards

See [`STYLEGUIDE.md`](STYLEGUIDE.md).

## Documentation standards

- New physics phenomena → new file under `docs/physics/`, not appended to
  an existing unrelated file.
- New AI/coaching features → new file under `docs/ai/`.
- Architecture changes → update `docs/architecture/Overview.md` and the
  relevant diagram under `docs/diagrams/`.
- Public API changes → doc comments in code **and** the matching page
  under `docs/api/`.

## Pull requests

- Keep PRs scoped to one crate or one concern where possible.
- Follow `.github/PULL_REQUEST_TEMPLATE.md`.
- Describe what changed, why, which docs/tests were updated, and which
  RFC (if any) the PR implements.
- CODEOWNERS review is required for anything under `crates/physics`,
  `crates/collision`, or `crates/spin`.

## Reporting bugs / requesting features

Use the templates under `.github/ISSUE_TEMPLATE/`. Physics bugs (a shot
that doesn't behave like the real world) should use the `physics.yml`
template so the report includes the reproduction data needed to write a
regression test.

## Code of conduct

Participation in this project is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
