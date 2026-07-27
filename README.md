# CueForge

**The Blender of Cue Sports**
*An Open Source Professional Cue Sports Simulation & Training Platform, built in Rust.*

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Rust](https://img.shields.io/badge/rust-stable-orange)]()

---

## What is CueForge?

CueForge is an open source platform for simulating, training, and analyzing cue sports (pool, snooker, carom billiards) with an emphasis on **physical accuracy**, **determinism**, and **extensibility**. It is built as a Cargo workspace of 15+ focused Rust crates, in the spirit of projects like Bevy, Tokio, and Rapier: small composable pieces, strong documentation, and an RFC process for architectural changes.

CueForge aims to be to cue sports what Blender is to 3D graphics: a single, free, professional-grade platform that individuals, coaches, researchers, and hobbyist developers can all build on.

---

## Project Status — Phase 1 Complete (M0 + M1)

Phase 1 (Milestone 0: Foundation & Milestone 1: Core Simulation) is fully initialized and implemented:

- **Deterministic Physics Core**: Rigid sphere impulse resolution, sliding/rolling friction, spin dynamics, cushion response, and pocket capture.
- **RFC 0001 & RFC 0002 Accepted**: Accepted specifications for physics engine foundation and spin dynamics model.
- **Physics Validation Test Suite**: Bit-identical determinism tests and analytical validation cases in `tests/physics/`.
- **Crate Architecture**: All 15 workspace crates (`common`, `physics`, `collision`, `spin`, `table`, `cue`, `rules`, `statistics`, `replay`, `renderer`, `ai`, `trainer`, `vision`, `networking`, `ui`) implemented and adhering strictly to the one-way dependency graph.
- **CLI Simulation App**: Runnable interactive simulation engine in `crates/ui`.

---

## Getting Started

### Prerequisites

- **Rust toolchain** (MSRV 1.75+): Install via [rustup.rs](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Verify

Clone the repository and run workspace checks:

```bash
git clone https://github.com/cueforge/cueforge.git
cd cueforge

# Format check
cargo fmt --check

# Clippy lints
cargo clippy --workspace -- -D warnings

# Run all tests & physics validation suite
cargo test --workspace
```

### Run the Interactive Simulation CLI

Launch the CueForge simulation demo to observe AI shot selection, deterministic physics stepping, ASCII table visualization, rule checking, and match statistics:

```bash
cargo run --bin cueforge
```

---

## Workspace Crate Overview

CueForge follows a strict one-way dependency graph: `common → physics → collision → spin → table → cue → rules → statistics → replay → renderer → vision → ai → trainer → networking → ui`.

| Crate | Purpose |
|---|---|
| [`cueforge-common`](crates/common) | Strongly-typed SI units (`Meters`, `Seconds`, ...), deterministic PCG PRNG (`RngState`), 2D/3D math vectors. |
| [`cueforge-physics`](crates/physics) | Simulation `World`, fixed-timestep integrator (1000 Hz), `Ball` state machine, and canonical event stream. |
| [`cueforge-collision`](crates/collision) | Continuous collision detection (CCD) and impulse resolution (normal + throw friction). |
| [`cueforge-spin`](crates/spin) | Spin/friction coupling (topspin, draw, english, swerve acceleration, spin decay). |
| [`cueforge-table`](crates/table) | 9ft regulation pool table geometry, cloth parameters, cushion rail response, pocket capture. |
| [`cueforge-cue`](crates/cue) | Cue stick parameters, stroke velocity, tip offset, squirt/deflection calculation, impulse transfer. |
| [`cueforge-rules`](crates/rules) | Game variant rules (8-ball, 9-ball, straight pool), foul detection, shot validity evaluation. |
| [`cueforge-statistics`](crates/statistics) | Match and shot metrics (pot accuracy, cue ball distance, foul tracking). |
| [`cueforge-replay`](crates/replay) | Deterministic shot recording and playback. |
| [`cueforge-renderer`](crates/renderer) | Text and ASCII table visualization for simulation state inspection. |
| [`cueforge-ai`](crates/ai) | Ghost ball aim angle calculation, shot search, and cut angle evaluation. |
| [`cueforge-trainer`](crates/trainer) | Training drills and practice scenarios. |
| [`cueforge-vision`](crates/vision) | Computer-vision tracking interface types. |
| [`cueforge-networking`](crates/networking) | Deterministic lockstep state hash checksums. |
| [`cueforge-ui`](crates/ui) | Interactive CLI app launcher. |

---

## Documentation

- [Architecture Overview](docs/architecture/Overview.md)
- [Physics White Paper](PHYSICS.md)
- [Validation Matrix](docs/physics/Validation.md)
- [Roadmap](ROADMAP.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [RFC Proposals](rfcs/)
- [Agent Rules](AGENTS.md)

---

## License

Dual-licensed under MIT or Apache-2.0, at your option. See [LICENSE](LICENSE).



### Verification Summary

  The full local verification suite was executed and passed with zero errors or lints:

  • Formatting: cargo fmt --check passed cleanly.
  • Clippy: cargo clippy --workspace -- -D warnings passed with 0 warnings across all 15
  crates.
  • Tests: cargo test --workspace passed 100% of unit, integration, and doc tests.
  ──────
  ### How to Start and Run the Project
  #### 1. Prerequisites

  Ensure Rust toolchain (MSRV 1.75+) is installed:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  #### 2. Run Verification Checks
  To verify the entire workspace, run:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace

  #### 3. Run the Interactive Simulation CLI

  Launch the CueForge simulation demo binary:

    cargo run --bin cueforge

  When run, the application will:

  1. Initialize a 9ft pool table with a cue ball and object balls.
  2. Render an initial ASCII table view.
  3. Consult cueforge-ai to solve for the ghost-ball target and optimal cut angle.
  4. Strike the cue ball and step the 1000 Hz physics simulation deterministically forward.
  5. Print the post-shot ASCII table state, event stream (collisions/pockets), rule
  evaluation (fouls), and match statistics.
