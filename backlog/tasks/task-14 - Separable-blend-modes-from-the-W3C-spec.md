---
id: TASK-14
title: Separable blend modes from the W3C spec
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-12
  - TASK-11
references:
  - https://www.w3.org/TR/compositing-1/
priority: medium
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The compositor shows only Normal. These 11 modes have published formulas (W3C Compositing and Blending Level 1).

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-render/src/blend/mod.rs` (new)
- `crates/editor-render/src/blend/separable.rs` (new)
- `crates/editor-render/src/lib.rs`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Multiply, Screen, Overlay, Darken, Lighten, Color Dodge, Color Burn, Hard Light, Soft Light, Difference, Exclusion are applied by the compositor
- [ ] #2 Per mode, a test checks a 5x5 grid of (0, 64, 128, 192, 255) pairs against the spec formula with at most 1 level of difference
- [ ] #3 Modes not yet implemented fall back to Normal and are listed in a `missing_modes()` function that a test prints
- [ ] #4 The W3C spec is logged in `docs/provenance-log.md`; `cargo xtask ci` passes
<!-- AC:END -->
