---
id: TASK-23
title: 'Catalog commands: select, show/hide and rename layers'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:catalog
  - agent:gemini
milestone: m-1
dependencies:
  - TASK-10
references:
  - docs/recipes/add-a-command.md
priority: medium
ordinal: 23000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Layers panel needs these commands; agents need them too.

**Track:** catalog · **Agent:** gemini

**Files allowed:**
- `crates/editor-engine/src/cmd/layer_select.rs` (new)
- `crates/editor-engine/src/cmd/layer_visibility.rs` (new)
- `crates/editor-engine/src/cmd/layer_rename.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `layer.select` makes a layer active (not a history step, per ADR-009)
- [ ] #2 `layer.visibility.toggle` flips visibility (a history step); `layer.rename` sets the name (1..=255 chars)
- [ ] #3 Each command: behavior test, undo test (where it is an Edit), bad-parameter test
- [ ] #4 Coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
