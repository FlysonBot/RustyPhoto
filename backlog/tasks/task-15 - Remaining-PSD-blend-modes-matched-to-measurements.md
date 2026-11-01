---
id: TASK-15
title: Remaining PSD blend modes, matched to measurements
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:gemini
milestone: m-1
dependencies:
  - TASK-14
  - TASK-13
references:
  - docs/behavior/blend-modes.md
priority: medium
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Linear Burn, Linear Dodge, Vivid Light, Linear Light, Pin Light, Hard Mix, Subtract, Divide, Darker Color, Lighter Color and Dissolve appear in real PSD files; without them those files display wrong.

**Track:** design · **Agent:** gemini

**Files allowed:**
- `crates/editor-render/src/blend/mod.rs`
- `crates/editor-render/src/blend/extra.rs` (new)
- `crates/editor-render/tests/blend_fixtures.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The 11 modes listed above are implemented
- [ ] #2 A fixture test compares all 65,536 pairs per mode with `tests/fixtures/blend/`: at least 99.9% within 1 level, maximum difference 2 (Dissolve: at most 1% of pixels differ in coverage)
- [ ] #3 Any mode that can't meet the tolerance is written up in the task with the measured error, not hidden by a looser test
- [ ] #4 `cargo xtask ci` passes
<!-- AC:END -->
