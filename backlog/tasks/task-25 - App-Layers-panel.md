---
id: TASK-25
title: 'App: Layers panel'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-11-01T01:04:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-21
  - TASK-23
  - TASK-24
  - TASK-11
priority: high
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Users can't see or manage layers. A docked panel on the right, like professional editors.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-app/src/panels/mod.rs` (new)
- `crates/editor-app/src/panels/layers.rs` (new)
- `crates/editor-app/src/app.rs` (one call)
- `crates/editor-app/src/lib.rs` (one `mod` line)
- `crates/editor-app/tests/layers_panel.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Lists layers top to bottom with a thumbnail, name, and eye toggle; click selects; the active layer is highlighted
- [ ] #2 Opacity, fill and blend mode controls for the active layer; buttons for new, duplicate and delete
- [ ] #3 Every action goes through `Engine::execute` with a catalog command id (no direct document edits)
- [ ] #4 Double-click on a name renames it in place
- [ ] #5 Snapshot with 3 layers attached and looked at; an egui_kittest test toggles visibility and checks the document
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
