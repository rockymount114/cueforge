## Summary

What does this PR change, and why?

## Related RFC / Issue

- RFC: (link, or "none — not an architectural change")
- Issue: (link, if applicable)

## Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] If touching `crates/physics`, `crates/collision`, or `crates/spin`:
      determinism/validation suite run and referenced below
      (`docs/physics/Validation.md`)
- [ ] Relevant docs under `docs/` updated in this PR (not deferred)
- [ ] No new dependency added to `crates/physics`, `crates/collision`, or
      `crates/spin` without a linked RFC

## Physics/validation notes (if applicable)

Which validation cases in `docs/physics/Validation.md` / `tests/physics/`
does this touch or add?

## Documentation updated

List the doc files updated alongside this change.
