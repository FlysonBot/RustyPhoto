---
id: TASK-18
title: 'editor-io crate: PNG read and write (gap G5)'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-10
references:
  - docs/decisions/ADR-010-codecs.md
priority: high
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The editor can't open or save anything. PNG first (M1 gate); JPEG and PSD follow in M2. ADR-010.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-io/` (new crate)
- `xtask/src/catalog.rs`
- `tests/fixtures/png/` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Reads 8-bit and 16-bit PNG in gray, gray+alpha, RGB, RGBA and palette forms (16-bit converted by round(v*255/65535)); writes RGBA8
- [ ] #2 Round trip write-then-read is exact on 20 random documents, including fully transparent pixels with color
- [ ] #3 A corpus of at least 10 bad files (truncated, wrong CRC, 0x0, 40000 wide, decompression bomb) returns errors, never panics; sizes are checked before allocating
- [ ] #4 A format registry makes `format.png` live; `cargo xtask catalog` counts registry ids as live
- [ ] #5 Crate is in `xtask/layers.toml` at layer 4; the `png` crate license is noted in the PR
- [ ] #6 `cargo xtask ci` passes
<!-- AC:END -->
