---
id: TASK-5
title: 'Catalog commands: Delete Layer and Duplicate Layer'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:catalog
  - agent:gemini
milestone: m-0
dependencies: []
references:
  - docs/recipes/add-a-command.md
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Basic layer management for M1, and a second agent family through the build-review-merge loop.

**Track:** catalog · **Agent:** gemini

**Files allowed:**
- `crates/editor-engine/src/cmd/layer_delete.rs` (new)
- `crates/editor-engine/src/cmd/layer_duplicate.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `layer.delete` removes the target layer (param `layer`, default active); the active index stays valid (the layer below, or 0)
- [ ] #2 Deleting the only layer returns an error and changes nothing
- [ ] #3 `layer.duplicate` inserts a copy directly above, named `<name> copy`, and makes it active
- [ ] #4 Tests check layer count, names, active index and pixels; undo restores the exact document for both
- [ ] #5 Coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
