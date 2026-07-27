# Testing

Testing strategy documentation, complementing the physics-specific
`docs/physics/Validation.md`.

## Planned contents

- `TestingStrategy.md` — overview of unit vs. integration vs. determinism
  vs. accuracy tests across the whole workspace (not just physics)
- `WritingDeterminismTests.md` — practical guide referencing
  `docs/physics/DeterministicSimulation.md`
- `CIMatrix.md` — explanation of the cross-platform CI matrix in
  `.github/workflows/ci.yml` and why it exists (determinism verification)

Status: minimal — expand as `tests/` grows beyond the physics validation
suite.
