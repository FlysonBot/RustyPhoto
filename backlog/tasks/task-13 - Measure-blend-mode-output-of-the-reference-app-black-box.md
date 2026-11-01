---
id: TASK-13
title: Measure blend-mode output of the reference app (black box)
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies: []
references:
  - docs/behavior/README.md
priority: medium
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Several PSD blend modes (vivid light, pin light, hard mix, darker color, dissolve...) have no published formula; we must match measured output. Black box only: never read any editor's code (AGENTS.md rule 8). Needs a browser session (the owner, or an agent with browser control).

**Track:** design · **Agent:** opus

**Files allowed:**
- `docs/behavior/blend-modes.md` (new)
- `tests/fixtures/blend/` (new)

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A test PSD or PNG pair is built: a 256-step gradient top layer over a 256-step gradient bottom layer (65,536 pairs), plus 50% opacity and 50% fill variants
- [ ] #2 For each of the 27 blend modes the reference app's flattened output is saved as a PNG fixture in `tests/fixtures/blend/`
- [ ] #3 `docs/behavior/blend-modes.md` follows `docs/behavior/README.md` (reference, method, result, confidence)
- [ ] #4 Every source is logged in `docs/provenance-log.md`
<!-- AC:END -->
