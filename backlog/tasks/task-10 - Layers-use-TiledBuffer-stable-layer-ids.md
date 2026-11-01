---
id: TASK-10
title: Layers use TiledBuffer; stable layer ids
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-11-01T01:04:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies:
  - TASK-9
  - TASK-8
  - TASK-1
references:
  - docs/decisions/ADR-006-document-model.md
priority: high
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Moves the document onto tiles (cheap undo, off-canvas pixels) and gives each layer a permanent id so agents and history stay correct when layers move. ADR-006 steps 1-3 (no groups yet).

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-doc/src/lib.rs`
- `crates/editor-doc/src/layer.rs` (new)
- `crates/editor-engine/src/cmd/*.rs` (existing commands)
- `crates/editor-engine/src/params.rs`
- `crates/editor-engine/src/preview.rs`
- `crates/editor-engine/src/state.rs`
- `crates/editor-engine/src/session.rs`
- `crates/editor-engine/src/lib.rs`
- `crates/editor-engine/tests/panic_hunt.rs`
- `crates/editor-app/src/canvas.rs`
- `docs/specs/state-json.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `Layer` holds a `TiledBuffer` and a `LayerId` that never changes and is never reused within a document
- [ ] #2 The `Layer` param kind accepts an id or an index; state JSON shows both
- [ ] #3 The active layer moves from `Document` to the `Session` as a `LayerId` (ADR-009); after undo, redo or any command a missing active layer falls back to the topmost layer (test covers deleting the active layer then undoing)
- [ ] #4 Every existing command and test is migrated with identical results
- [ ] #5 A 6000x4000 document with 20 layers: running Invert on one layer then cloning the document for history adds no more than that one layer's tiles (test checks `unique_bytes`)
- [ ] #6 No code outside `editor-doc` indexes raw pixel vectors
- [ ] #7 `cargo xtask ci` passes
<!-- AC:END -->
