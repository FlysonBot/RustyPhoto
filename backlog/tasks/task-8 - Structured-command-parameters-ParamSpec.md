---
id: TASK-8
title: Structured command parameters (ParamSpec)
status: To Do
assignee: []
created_date: '2026-10-31T23:45:00Z'
labels:
  - track:design
  - agent:opus
milestone: m-1
dependencies:
  - TASK-7
references:
  - docs/recipes/add-a-command.md
priority: high
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Parameters are documented as free text today, so the UI can't build a dialog and agents must parse prose. Typed parameter specs feed dialogs (gap G3), agents and the fuzz test from one source.

**Track:** design · **Agent:** opus

**Files allowed:**
- `crates/editor-engine/src/cmd/mod.rs` (structural change)
- `crates/editor-engine/src/params.rs`
- `crates/editor-engine/src/cmd/*.rs` (existing commands)
- `crates/editor-engine/src/state.rs`
- `crates/editor-engine/tests/panic_hunt.rs`
- `docs/recipes/add-a-command.md`
- `docs/specs/state-json.md`

Shared hot spots also allowed (AGENTS.md section 6: re-read first, one-line edits, regenerate on conflict): Cargo.lock; Cargo.toml ([workspace.dependencies] lines); catalog/floor.txt; crates/editor-engine/src/cmd/mod.rs (command_specs list); docs/gaps.md (own gap entry); docs/generated/coverage.md; docs/provenance-log.md (append); xtask/layers.toml ([layers] line).

Need another file? Stop and explain why in this task (docs/prompts/03-implement.md step 4).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `CommandSpec::params` is a list of `ParamSpec { name, label, kind, help }`; kinds: Int{min,max,default}, Float{min,max,default}, Bool, Choice{options,default}, Text{max_len}, Color, Path{save: bool}, Layer
- [ ] #2 `Params` getters check values against the spec; a value outside the range is an error (no silent clamping) and the error names the parameter
- [ ] #3 The fuzz test generates at least 6 hostile values per declared parameter from its kind (out of range, wrong type, NaN, huge, empty, null)
- [ ] #4 State JSON lists parameter specs for every command; spec updated
- [ ] #5 All existing commands converted; recipe updated; `cargo xtask ci` passes
<!-- AC:END -->
