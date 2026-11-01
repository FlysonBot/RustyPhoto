---
id: TASK-22
title: 'App: canvas zoom and pan, transparency checkerboard, View zoom commands'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-1
  - TASK-10
priority: medium
ordinal: 22000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The canvas only shrinks to fit and transparent areas look white. Users need to zoom, pan and see transparency.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-app/src/canvas.rs`
- `crates/editor-engine/src/cmd/view_zoom.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `view.zoom_in`, `view.zoom_out`, `view.fit_on_screen`, `view.actual_pixels` are live Session commands changing the session zoom (steps: 1/16 to 32x, preset ladder)
- [ ] #2 Mouse wheel with Ctrl zooms around the cursor; Space+drag and middle-drag pan
- [ ] #3 Transparent pixels show a gray checkerboard (8 px squares at 100%)
- [ ] #4 Above 400% zoom, pixels are drawn sharp (nearest neighbor)
- [ ] #5 Snapshot at 25%, 100% and 800% attached and looked at; coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
