# CueForge

**The Blender of Cue Sports**  
*An Open Source Professional Cue Sports Simulation & Training Platform, built in Rust.*

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)]()
[![Rust](https://img.shields.io/badge/rust-stable-orange)]()

---

## What is CueForge?

CueForge is an open source platform for simulating, training, and analyzing cue sports (pool, snooker, carom billiards) with an emphasis on **physical accuracy**, **determinism**, and **extensibility**. It is built as a Cargo workspace of 15 focused Rust crates, in the spirit of projects like Bevy, Tokio, and Rapier: small composable pieces, strong documentation, and an RFC process for architectural changes.

CueForge aims to be to cue sports what Blender is to 3D graphics: a single, free, professional-grade platform that individuals, coaches, researchers, and hobbyist developers can all build on.

---

## How to Start & Run the Project

### 1. Prerequisites

Ensure you have the Rust toolchain (MSRV 1.75+) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Run the Interactive Simulation CLI

Launch the CueForge command-line simulation demo to observe AI shot selection, deterministic physics stepping, ASCII table visualization, rule checking, and match statistics:

```bash
cargo run --bin cueforge
```

When started, the application will:
1. Initialize a 9ft pool table with cue ball and object balls.
2. Render an ASCII table representation of the table bed and ball locations.
3. Consult `cueforge-ai` to calculate the ghost-ball target, aim azimuth, and cut angle.
4. Strike the cue ball and step the 1000 Hz simulation forward deterministically.
5. Print the post-shot table state, event stream (collisions/pockets), rule foul evaluations, and match statistics.

### 3. Launch CueForge Studio (Web UI)

CueForge Studio provides an interactive 2D graphical web interface for shot experimentation, ball placement, and real-time visualization:

**Option A — Using Python static HTTP server:**
```bash
python3 -m http.server 8080 --directory web
```
Then open [http://localhost:8080](http://localhost:8080) in your web browser.

**Option B — Direct Browser Open:**
```bash
# macOS
open web/index.html

# Linux
xdg-open web/index.html
```

#### Key Web UI Features:
- **Interactive 2D Table Viewport**: Aiming vector line, target ball cut angle indicator, and ghost ball position guide.
- **Ball in Hand Drag Placement**: Click *"Place Cue Ball"* to reposition cue ball freely anywhere on the table bed. Automatic cue ball respawn on scratch.
- **Tip Contact Selector (English Spin)**: 2D target widget to adjust topspin, draw, and left/right side spin.
- **AI Coach Advisor**: Click *"AI Coach Shot"* for instant shot recommendations and cut angle difficulty scoring.
- **Physics Event Stream**: Live sidebar log displaying real-time collision and pocket capture events.

### 4. Local Build & Verification

To verify code formatting, run Clippy lints, and execute the full test suite across all 15 crates:

```bash
# Code formatting check
cargo fmt --check

# Clippy lints (warnings treated as errors)
cargo clippy --workspace -- -D warnings

# Unit tests, integration tests & physics validation suite
cargo test --workspace
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
| [`cueforge-statistics`](crates/statistics) | Match and shot metrics (pot accuracy, cue ball distance, foul tracking, landing heatmaps). |
| [`cueforge-replay`](crates/replay) | Deterministic shot recording, keyframe seeking, and playback controller. |
| [`cueforge-renderer`](crates/renderer) | Text and ASCII table visualization for simulation state inspection. |
| [`cueforge-ai`](crates/ai) | Ghost ball aim angle calculation, shot search, position prediction, and safety evaluation. |
| [`cueforge-trainer`](crates/trainer) | Interactive training drills catalog (Wagon Wheel, Straight Line) and score evaluator. |
| [`cueforge-vision`](crates/vision) | Computer-vision tracking pipeline and 4-point table homography calibration. |
| [`cueforge-networking`](crates/networking) | Deterministic lockstep state hash checksums and desync detection. |
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
