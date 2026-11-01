---
id: TASK-9
title: Tiled pixel storage (TiledBuffer)
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies: []
references:
  - docs/decisions/ADR-004-history-storage.md
  - docs/decisions/ADR-006-document-model.md
priority: high
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Whole-layer pixel vectors make undo copy entire documents (96 MB per 6000x4000 layer) and can't hold pixels outside the canvas. Shared copy-on-write tiles fix both. Standalone type; nothing uses it yet. ADR-004 and ADR-006.

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-doc/src/tiles.rs` (new)
- `crates/editor-doc/src/lib.rs` (one `pub mod` line)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `TiledBuffer` stores 256x256 RGBA8 tiles in `Arc`, sparse (missing tile = transparent), with signed tile coordinates
- [ ] #2 API: get/set pixel, read/write a rectangle, iterate tiles, `unique_bytes` over a set of buffers counting shared tiles once
- [ ] #3 Cloning a buffer holding a full 6000x4000 image allocates no pixel memory; setting one pixel afterwards copies exactly one tile (tests check `Arc` counts)
- [ ] #4 A randomized test of 1000 operations matches a flat `Vec` reference model exactly
- [ ] #5 Coordinates far outside (i32 extremes) return errors or transparent, never panic
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
