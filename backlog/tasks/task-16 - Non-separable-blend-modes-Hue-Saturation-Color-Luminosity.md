---
id: TASK-16
title: 'Non-separable blend modes: Hue, Saturation, Color, Luminosity'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-15
references:
  - https://www.w3.org/TR/compositing-1/
priority: low
ordinal: 16000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
These four modes mix whole colors (not channel by channel) and complete the PSD blend mode set.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-render/src/blend/mod.rs`
- `crates/editor-render/src/blend/nonseparable.rs` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The 4 modes follow the W3C formulas (SetLum, ClipColor, SetSat)
- [ ] #2 Tests with at least 20 hand-computed color pairs per mode, plus the measured fixtures within 2 levels for 99% of pairs
- [ ] #3 `missing_modes()` is empty; `cargo xtask ci` passes
<!-- AC:END -->
