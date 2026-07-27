# API Reference

This directory is reserved for generated/curated public API documentation
for each crate, supplementing (not replacing) `cargo doc` output.

## Planned contents

One file per public-facing crate once its API stabilizes, e.g.:

- `physics.md` — `crates/physics` public API narrative (the `World`/`step`
  contract described in RFC 0001)
- `plugin-api.md` — the plugin trait(s) described in
  `docs/architecture/PluginSystem.md`

## Viewing the CueForge Interface UI

CueForge provides both a graphical Web UI interface and a CLI terminal inspector for manual testing and simulation visualization:

### 1. Interactive Graphical Web UI (CueForge Studio)

A 60 FPS interactive 2D Canvas Web application (`web/`) featuring regulation 9ft table geometry, mouse aiming, ghost-ball trajectory prediction, cue power/elevation sliders, tip contact spin selector, AI shot advice, and real-time physics event logging.

- **URL**: `http://localhost:8080`
- **Serve Command**:
  ```bash
  python3 -m http.server 8080 --directory web
  ```

### 2. Terminal ASCII Visualizer

A lightweight text-based ASCII table renderer for quick terminal-based testing (`cueforge-ui` crate).

- **Execution Command**:
  ```bash
  cargo run --bin cueforge
  ```

## Policy

Per `AGENTS.md` §7 / `CONTRIBUTING.md`: any public API change must update
the matching page here in the same PR, not as a follow-up.
