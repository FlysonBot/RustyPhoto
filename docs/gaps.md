# Gaps: the ranked work queue

Status: replace this line, don't append to it · Last updated: 2026-10-31 (M0+M1 plan)

Agents pick the highest-ranked entry nobody holds (create a Backlog.md task
for it first). When you close part of a gap, update its Done/Evidence lines.
Every number here is *estimated* unless it links to a generated file.

## G1. Agents can't see the UI
Done:     `--snapshot out.png --size WxH` flag and snapshot tests (egui_kittest), TASK-1, 2026-10-31
Done:     snapshot tests run on CI on every OS (Linux via Mesa's software renderer), TASK-29, 2026-10-31
Evidence: `crates/editor-app/tests/snapshot.rs`, `docs/recipes/check-ui.md`
Impact:   every UI task; blocks agents from checking their own work
Tasks:    TASK-1, TASK-29 (ADR-008)
Human:    ADR-008 accepted 2026-10-31; CI change approved 2026-10-31

## G2. No agent control channel
Missing:  a way for agents to drive the engine and inspect the document as JSON (CLI first, MCP server in M2)
Impact:   end-to-end testing by agents
Tasks:    TASK-2 (state JSON), TASK-3 (CLI), TASK-26 (CLI open/render) (ADR-007)
Human:    none (ADR-007 accepted 2026-10-31)

## G3. Commands can't take parameters from the UI
Missing:  typed parameter specs; dialogs generated from them; keyboard shortcuts from catalog
Impact:   every adjustment and filter with settings
Tasks:    TASK-8 (ParamSpec), TASK-21 (dialogs), TASK-20 (shortcuts)
Human:    owner approves the dialog look once (design track)

## G4. No real compositor
Missing:  `editor-render` crate: tiles, blend modes, opacity/fill, masks, groups, tile cache
Partial:  Normal blend placeholder in `crates/editor-engine/src/preview.rs`
Impact:   correctness of everything shown on screen
Tasks:    TASK-12 (Normal, tiles), TASK-14/15/16 (blend modes), TASK-13 (measurements). Masks, groups, cache: M2
Human:    ADRs accepted 2026-10-31; TASK-13 needs a browser session with the reference app

## G5. No file I/O
Missing:  open/save PNG (M1); JPEG, PSD read with the embedded composite as an oracle, PSD write (M2)
Impact:   the editor is unusable without it
Tasks:    TASK-18 (PNG codec), TASK-19 (New/Open/Export commands) (ADR-010)
Human:    owner provides sample PSDs saved by the reference app (needed in M2)

## G6. Document model and history don't scale
Missing:  tiled copy-on-write layers, stable layer ids, memory-budget history with timestamps
Evidence: today undo keeps 50 full document copies (`crates/editor-engine/src/lib.rs`)
Impact:   the 6000×4000, 20-layer speed and memory targets in the M2 gate
Tasks:    TASK-9, TASK-10, TASK-11, TASK-17 (ADR-004, ADR-006)
Human:    none (ADR-004, ADR-006 accepted 2026-10-31)

## G7. Non-edit features can't be commands
Missing:  command effects (edit / session / read-only / replace) and session state (view, colors, tool)
Impact:   zoom, colors, tools, open and export can't be live or reachable by agents
Tasks:    TASK-7 (ADR-009), TASK-22 (canvas zoom/pan)
Human:    none (ADR-009 accepted 2026-10-31)

## G8. Catalog needs the owner's review
Done:     2026-10-31: expanded from 52 seed entries to the full target list (see `docs/generated/coverage.md`)
Done:     2026-10-31: owner approved the outline
Missing:  nothing; close this entry at the next gaps rewrite
