---
id: TASK-28
title: Sync docs at end of M1
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:catalog
  - agent:gemini-batch
milestone: m-1
dependencies:
  - TASK-27
  - TASK-25
  - TASK-22
  - TASK-21
  - TASK-16
  - TASK-6
priority: low
ordinal: 28000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tasks change code faster than the overview docs. After M1, the architecture page, glossary and status must describe what exists (one home per fact, AGENTS.md section 7).

**Track:** catalog · **Agent:** gemini-batch

**Files allowed:**
- `docs/architecture.md`
- `docs/glossary.md`
- `docs/status.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `docs/architecture.md` crate table and command flow match the code (every crate in `xtask/layers.toml` listed)
- [ ] #2 Placeholder table lists only placeholders that still exist
- [ ] #3 Glossary has entries for tile, session, effect, LayerId, ParamSpec
- [ ] #4 `cargo xtask docpaths` and `cargo xtask ci` pass
<!-- AC:END -->
