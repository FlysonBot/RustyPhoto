---
id: TASK-11
title: Layer blend mode, fill and lock fields
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
updated_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-10
references:
  - docs/decisions/ADR-006-document-model.md
priority: medium
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The compositor and the Layers panel need each layer's blend mode, fill and locks, matching the PSD layer record. Adds the fields and the panel commands that set them.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-doc/src/layer.rs`
- `crates/editor-doc/src/blend.rs` (new)
- `crates/editor-engine/src/cmd/layer_props.rs` (new)
- `crates/editor-engine/src/state.rs`
- `docs/specs/state-json.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `BlendMode` enum lists the 27 PSD layer blend modes plus Pass Through (groups) with their PSD 4-character keys; round trip key to enum to key is exact for all
- [ ] #2 `Layer` gains `blend_mode` (default Normal), `fill` (0.0..=1.0, default 1.0), locks (transparency, pixels, position)
- [ ] #3 Catalog commands `layer.blend_mode.set`, `layer.opacity.set`, `layer.fill.set`, `layer.lock.transparency`, `layer.lock.pixels`, `layer.lock.position` are live, each with undo and bad-parameter tests
- [ ] #4 State JSON shows the new fields
- [ ] #5 Coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
