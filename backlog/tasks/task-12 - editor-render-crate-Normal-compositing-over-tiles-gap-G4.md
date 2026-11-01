---
id: TASK-12
title: 'editor-render crate: Normal compositing over tiles (gap G4)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies:
  - TASK-10
references:
  - docs/decisions/ADR-005-compositor.md
  - docs/decisions/ADR-002-pixel-format-and-alpha.md
priority: high
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The placeholder compositor redoes every pixel every frame and lives in the wrong crate. This is the real compositor's base: per-tile, premultiplied float math, region rendering. ADR-002 and ADR-005.

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-render/` (new crate)
- `crates/editor-engine/src/preview.rs`
- `crates/editor-engine/Cargo.toml`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `editor_render::composite(&Document, rect)` returns straight RGBA8 for any rectangle; full image = whole canvas
- [ ] #2 Straight to premultiplied and back is exact for every alpha 1..=255 and every channel value (exhaustive test)
- [ ] #3 Normal blend with opacity and visibility; at least 8 golden tests with exact expected pixels (including half-transparent over half-transparent)
- [ ] #4 Rendering any 64x64 region equals the same crop of the full render, on 5 random documents
- [ ] #5 `preview.rs` becomes a thin call into `editor-render` (app code unchanged); crate is in `xtask/layers.toml` at layer 3
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
