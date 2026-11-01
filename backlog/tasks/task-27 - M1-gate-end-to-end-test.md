---
id: TASK-27
title: 'M1 gate: end-to-end test'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-26
  - TASK-17
  - TASK-23
  - TASK-14
priority: high
ordinal: 27000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The M1 gate is: open, layer, edit, undo, save PNG, compositor tests green. One test that walks the whole path proves it and keeps it working.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-cli/tests/m1_gate.rs` (new)
- `tests/fixtures/m1/` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One test: open a PNG fixture, add a layer, fill or invert, set a blend mode, undo and redo, export PNG; the exported pixels equal a stored expected PNG exactly
- [ ] #2 The same steps run through `editor-cli --script` and through the engine API give identical output
- [ ] #3 All `editor-render` tests pass
- [ ] #4 `cargo xtask ci` passes
<!-- AC:END -->
