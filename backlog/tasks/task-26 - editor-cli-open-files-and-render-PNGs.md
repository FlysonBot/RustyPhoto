---
id: TASK-26
title: 'editor-cli: open files and render PNGs'
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:sonnet
milestone: m-1
dependencies:
  - TASK-3
  - TASK-19
references:
  - docs/decisions/ADR-007-agent-control.md
priority: medium
ordinal: 26000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Agents can drive the engine but can't load images or see results. Rendering to PNG lets them check their work.

**Track:** design · **Agent:** sonnet

**Files allowed:**
- `crates/editor-cli/src/`
- `crates/editor-cli/Cargo.toml`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `run --open in.png` starts from a file; `--render out.png` writes the composite after the steps
- [ ] #2 `editor-cli render in.png out.png` exists as a shortcut
- [ ] #3 Integration tests open a fixture, invert, render, and compare exact pixels
- [ ] #4 `cargo xtask ci` passes
<!-- AC:END -->
