# Architecture

Status: current state only · Last reviewed: 2026-10-31

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

| Crate | Layer | Responsibility | Task / decision |
| --- | --- | --- | --- |
| `editor-render` | L3 | Tile compositor, blend modes | TASK-12, ADR-005 |
| `editor-io` | L4 | File formats (PNG in M1) | TASK-18, ADR-010 |
| `editor-cli` | L6 (no GUI) | Headless control for agents and tests | TASK-3, ADR-007 |
| `editor-algo` | L2 | Filters and adjustments math | M2 |
| `editor-platform` | L6 | OS-specific code | later |

## Agreed next steps (M1; ADRs accepted by the owner 2026-10-31)

- **Pixels** (ADR-002, ADR-003): straight-alpha RGBA, 8 bits, sRGB in the
  document; premultiplied `f32` only inside the compositor.
- **Document** (ADR-006): layers become sparse grids of shared 256×256
  copy-on-write tiles and get a permanent `LayerId`.
- **History** (ADR-004): each step is a whole-document snapshot that shares
  unchanged tiles; a memory budget (default 2 GiB) replaces the 50-step limit.
- **Commands** (ADR-009): every command has an effect (Edit, Session,
  ReadOnly, Replace); only Edit records history. Parameters become typed
  `ParamSpec` lists that drive dialogs, agents and the fuzz test.
- **Agents** (ADR-007, ADR-008): a state JSON format, a headless CLI, and
  offscreen UI snapshots.

## How a command runs

1. The user clicks a menu item (or an agent or test calls the engine) with an id.
2. `Engine::execute` finds the `CommandSpec`, checks `enabled`.
3. It runs the command on a **copy** of the document.
4. Success: the copy becomes the document and the old one goes on the undo
   stack (exactly one undo step per command). Failure: nothing changes.
5. The app marks the canvas dirty and redraws.

Undo currently stores whole-document snapshots (simple, memory-hungry,
50 steps). Tile-sharing history is planned (ADR-004, TASK-17).

## Known placeholders

| Placeholder | Where | Replace with |
| --- | --- | --- |
| Normal-blend-only compositor | `crates/editor-engine/src/preview.rs` | `editor-render` crate (TASK-12) |
| Commands run with default parameters only | `crates/editor-app/src/main.rs` | Dialogs generated from `ParamSpec` (TASK-8, TASK-21) |
| No keyboard shortcuts | app | Shortcut dispatch from catalog `shortcut` fields (TASK-20) |
| Fixed 800×600 white document at start | app | File > New / Open (TASK-19) |
