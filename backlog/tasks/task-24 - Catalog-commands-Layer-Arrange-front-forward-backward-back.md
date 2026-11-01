---
id: TASK-24
title: 'Catalog commands: Layer > Arrange (front, forward, backward, back)'
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
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Reordering layers is basic layer management (vision: move up/down).

**Track:** catalog · **Agent:** gemini

**Files allowed:**
- `crates/editor-engine/src/cmd/layer_arrange.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `layer.arrange.front`, `.forward`, `.backward`, `.back` move the target layer; layer ids are unchanged and the moved layer stays active
- [ ] #2 Moving the top layer forward (or bottom backward) is a no-op that records no step, or a clear error: pick one and test it
- [ ] #3 Tests on a 4-layer document check the exact order for all 4 commands; undo restores the exact document
- [ ] #4 Coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
