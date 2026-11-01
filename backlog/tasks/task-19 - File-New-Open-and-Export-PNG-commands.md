---
id: TASK-19
title: File > New, Open and Export PNG commands
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-18
  - TASK-8
  - TASK-12
references:
  - docs/decisions/ADR-009-command-effects.md
priority: high
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
With the PNG codec in place, users and agents need commands to start a document, open a file and export the flattened image.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-engine/src/cmd/file_new.rs` (new)
- `crates/editor-engine/src/cmd/file_open.rs` (new)
- `crates/editor-engine/src/cmd/file_export_png.rs` (new)
- `crates/editor-engine/Cargo.toml`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `file.new` (Replace): width, height (1..=30000), fill choice (white, black, transparent, background color), and presets (at least 6, e.g. 1920x1080, A4 300 dpi)
- [ ] #2 `file.open` (Replace): `path` param; opens PNG; history cleared; a bad file returns an error and keeps the current document
- [ ] #3 `file.export.png` (ReadOnly): `path` param; writes the flattened composite from `editor-render`; no history step
- [ ] #4 Tests: new then export then open gives identical pixels; open of a missing file is an error
- [ ] #5 Coverage regenerated and floor raised; `cargo xtask ci` passes
<!-- AC:END -->
