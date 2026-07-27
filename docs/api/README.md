# API Reference

This directory is reserved for generated/curated public API documentation
for each crate, supplementing (not replacing) `cargo doc` output.

## Planned contents

One file per public-facing crate once its API stabilizes, e.g.:

- `physics.md` — `crates/physics` public API narrative (the `World`/`step`
  contract described in RFC 0001)
- `plugin-api.md` — the plugin trait(s) described in
  `docs/architecture/PluginSystem.md`

## Policy

Per `AGENTS.md` §7 / `CONTRIBUTING.md`: any public API change must update
the matching page here in the same PR, not as a follow-up.

Status: empty — populate as each crate's public API is implemented and
stabilized.
