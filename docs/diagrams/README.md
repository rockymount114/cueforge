# Diagrams

Source and rendered versions of architecture/physics diagrams referenced
throughout `docs/`. CueForge prefers Mermaid (renders natively on GitHub,
diffable as text) over binary diagram formats where possible.

## Current diagrams

- Layer diagram — embedded directly in `docs/architecture/Overview.md`
  and `docs/architecture/DataFlow.md` (Mermaid, inline)
- World-frame diagram — embedded in `docs/physics/CoordinateSystem.md`
  (Mermaid, inline)

## Policy

New diagrams should be added as Mermaid blocks directly in the relevant
doc file where possible; this directory is for diagrams that don't belong
to a single doc page (e.g. a full-repository dependency graph) or for
non-Mermaid source files (e.g. a Draw.io `.drawio` source) with an
exported `.svg` alongside it.

Status: minimal — most diagrams currently live inline in their owning
doc page rather than here.
