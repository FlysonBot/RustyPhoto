# Architecture

Status: current state only · Last reviewed: (date)

Superseded designs move to `docs/decisions/` as replaced ADRs; this file
describes only what exists now plus the agreed next steps.

## The three structural rules

1. **The target is data.** `catalog/catalog.toml` lists every feature the
   finished editor should have, each with a stable id. Coverage is computed
   (`docs/generated/coverage.md`), never estimated.
2. **Every feature is a command.** A `CommandSpec` (id, label, params doc,
   enabled check, run function) in `crates/editor-engine/src/cmd`. Menus, tests,
   scripts and agents all call `Engine::execute(id, params)`.
3. **Layers are enforced.** `xtask/layers.toml` gives each crate a layer;
   dependencies point strictly downward; only L6 uses GUI crates.

## Crates

| Crate | Layer | Responsibility | Depends on |
| --- | --- | --- | --- |
| `editor-doc` | L0 | `Document`, `Layer`, `Rgba8` pixels, `DocError` | nothing |
| `editor-catalog` | L0 | Parses and embeds `catalog/catalog.toml` | nothing |
| `editor-engine` | L5 | `Engine` (document + undo/redo), command registry, commands, placeholder compositor (`preview.rs`) | editor-doc |
| `editor-app` | L6 | eframe window, catalog-driven menus, canvas, status bar | all above |
| `xtask` | tool | `ci`, `layers`, `catalog`, `docpaths` | engine, catalog |

Planned crates (added by tasks, each with a line in `xtask/layers.toml`):
`editor-render` (L3, real compositor and blend modes), `editor-io` (L4, file
formats), `editor-algo` (L2, filters and adjustments), `editor-platform`
(L6, OS-specific code).

## How a command runs

1. The user clicks a menu item (or an agent or test calls the engine) with an id.
2. `Engine::execute` finds the `CommandSpec`, checks `enabled`.
3. It runs the command on a **copy** of the document.
4. Success: the copy becomes the document and the old one goes on the undo
   stack (exactly one undo step per command). Failure: nothing changes.
5. The app marks the canvas dirty and redraws.

Undo currently stores whole-document snapshots (simple, memory-hungry).
Tile-based history is a planned decision (see `docs/gaps.md`).

## Known placeholders

| Placeholder | Where | Replace with |
| --- | --- | --- |
| Normal-blend-only compositor | `crates/editor-engine/src/preview.rs` | `editor-render` crate |
| Commands run with default parameters only | `crates/editor-app/src/main.rs` | Dialogs generated from `CommandSpec::params` |
| No keyboard shortcuts | app | Shortcut dispatch from catalog `shortcut` fields |
| Fixed 800×600 white document at start | app | File > New / Open |
