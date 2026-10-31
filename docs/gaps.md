# Gaps: the ranked work queue

Status: replace this line, don't append to it · Last updated: (date)

Agents pick the highest-ranked entry nobody holds (create a Backlog.md task
for it first). When you close part of a gap, update its Done/Evidence lines.
Every number here is *estimated* unless it links to a generated file.

## G1. Agents can't see the UI
Missing:  offscreen snapshot (render the app to PNG without a window, e.g. egui_kittest); `--snapshot out.png` flag
Evidence: AGENTS.md rule 6 is a stopgap
Impact:   every UI task; blocks agents from checking their own work
Human:    none

## G2. No agent control channel
Missing:  a way for agents to drive the running app and inspect the document as JSON (CLI first, MCP server later)
Impact:   end-to-end testing by agents
Human:    none

## G3. Commands can't take parameters from the UI
Missing:  dialogs generated from `CommandSpec::params`; keyboard shortcuts from catalog
Impact:   every adjustment and filter with settings
Human:    owner approves the dialog look once (design track)

## G4. No real compositor
Missing:  `editor-render` crate: blend modes, opacity/fill, masks, groups, tiling
Partial:  Normal blend placeholder in `crates/editor-engine/src/preview.rs`
Impact:   correctness of everything shown on screen
Human:    owner approves pixel-format ADR (straight vs premultiplied, 8/16-bit)

## G5. No file I/O
Missing:  open/save PNG and JPEG; then PSD reading with the embedded composite as an oracle
Impact:   the editor is unusable without it
Human:    owner provides sample PSDs saved by real Photoshop

## G6. Catalog is a seed
Missing:  the full target list (expect several hundred entries)
Impact:   honest progress numbers; parallel work supply
Human:    owner reviews the menu outline once
