---
id: TASK-6
title: Record M0 gate evidence in docs/status.md
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:catalog
  - agent:gemini-batch
milestone: m-0
dependencies:
  - TASK-4
priority: low
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M0 is done when one catalog command was built, reviewed and merged by agents and CI is green on Linux, Windows and macOS. The evidence (PR link, CI run links) must be written down so the owner can close the milestone.

**Track:** catalog · **Agent:** gemini-batch

**Files allowed:**
- `docs/status.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `docs/status.md` Today section links the merged PR for the flip commands and its cross-family review
- [ ] #2 It links the first `main` CI run after that merge, with 3 green jobs (ubuntu, windows, macos)
- [ ] #3 No numbers are copied from coverage.md (link only)
<!-- AC:END -->
