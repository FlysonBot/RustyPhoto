# Vision

Status: DRAFT · Last updated: 2026-10-31 · Owner approval: (not yet)

> Changes after approval are deliberate decisions, recorded in the log at the bottom.

## Who it's for

1. People who want a free, local, layer-based photo editor that feels like the
   professional editors they already know.
2. People who want their AI agent to edit photos for them. For now that agent
   is **Claude Code only**. Other agents may come later.

## What the first usable version (M2) must do

**Files**
- Open PNG, JPEG and PSD. "PSD" means 8-bit RGB files saved by Photopea (the
  owner's reference app; we have no Photoshop to test with).
- Save as PSD with layers. Export PNG and JPEG as one flattened image.
- Saving never throws away PSD content we don't support yet (for example
  text, smart objects or layer styles). Content the user didn't touch is
  written back unchanged. If the user edits a layer we can't fully represent,
  the app warns them first that the layer will become plain pixels.
- New blank canvas of any size, with suggested presets.

**Layers**: add, delete, move up/down, show/hide, rename. Groups, opacity and
blend modes from PSDs are kept and displayed correctly.

**Tools**
- Brush and eraser with size, hardness and opacity.
- Rectangle, ellipse and lasso selection.
- Copy, cut, paste and delete inside a selection. Paste makes a new layer.
- Fill (paint bucket) with a tolerance setting and an "only connected area" toggle.
- Eyedropper and color chooser.
- Full transparency everywhere.

**Adjustment layers**: Brightness/Contrast, Hue/Saturation, Levels. Each can be
edited again later and saves into the PSD.

**History**: no limit on the number of steps; only a memory budget (default
2 GB, adjustable). When the budget runs out, the oldest steps are dropped with
a notice. A History panel lists every step, with a toggle that shows when each
step happened. History is cleared when the file is closed.

**Agent control (MCP)**: Claude Code can do anything a user can, over MCP:
- run every live menu command;
- read the document's state (layers, selection, history) as structured data;
- get a rendered PNG of the canvas;
- paint brush strokes, sent as a list of points.

## What it will NOT do (for now)

- No built-in AI that generates or edits pixels by itself. AI help always comes
  from the user's own Claude Code.
- No web version, ever (pure native app).
- No plugins yet, but the command system should make them easy to add later.
- No pen-tablet pressure (the owner can't test it). Pressure *simulated from
  mouse speed* may be considered after M2.
- No history branches yet (finding edits you undid and then replaced). Planned
  for a later version.
- No 16-bit color or CMYK in M2.
- No agents other than Claude Code in M2.

## How it should feel

- Familiar to people who use professional editors: same layout ideas, same
  tool names, same shortcuts where possible. (Other products' names never
  appear in the app itself: `AGENTS.md` rule 10.)
- Smooth: see the measurable speed targets in the M2 gate.
- Dark UI.
- Later (after M2): a built-in Claude Code side panel, in the style of the
  Claude Code extension for VS Code: a proper chat panel, not a terminal.
  Claude Code edits the open image through MCP while the user watches.

## Fixed decisions

- Pure Rust, native desktop app for Linux, Windows and macOS. GUI: egui (`docs/decisions/ADR-001-gui-library.md`).
- Clean room: no other editor's source code is ever read (`AGENTS.md` rule 8).
  Comparing *outputs* (for example opening our PSDs in Photopea) is allowed.
- Built by agents: Claude Code and Google Antigravity, tasks in Backlog.md.

## Milestones

| Milestone | Means | Gate (checked by) |
| --- | --- | --- |
| M0 Setup | The loop works end to end | One catalog command built, reviewed and merged by agents; CI green on 3 OSes |
| M1 Foundation | A real editor core | Open, layer, edit, undo, save PNG; compositor tests green |
| M2 Breadth | Usable for simple work | Every box in the M2 gate below is ticked |
| M3 Hard features | Layer styles, text, brushes, smart objects; Claude Code side panel | PSD fidelity score target set at M3 start |
| M4 Release | Installers for 3 OSes | Signed builds; owner sign-off |

### M2 gate (all must be true)

Automatic (checked by tests in CI):
- [ ] Every M2 feature listed above is a live command in `docs/generated/coverage.md`.
- [ ] Open PNG, JPEG and PSD; export PNG and JPEG; save PSD.
- [ ] Every sample PSD in the owner's test set (saved by Photopea) opens
      without a crash. Its on-screen result matches the PSD's embedded
      preview image within a tolerance set by a test.
- [ ] Save then reopen in our app gives identical layers and pixels.
- [ ] Save keeps unsupported PSD content: it comes back byte-for-byte unchanged
      when the user didn't touch it.
- [ ] Undo/redo of every command restores the exact previous document.
- [ ] Every live command can be run over MCP; MCP can read document state and
      a rendered PNG (one test walks the full command list).
- [ ] Crash fuzz test (`crates/editor-engine/tests/panic_hunt.rs`) passes.
- [ ] Speed, with a 6000×4000 image and 20 layers on the CI machine (numbers
      adjusted to a mid-range laptop): opens in under 3 s, a brush stroke
      updates at 60 frames per second, undo takes under 100 ms.

Owner (checked by hand, once):
- [ ] Three PSDs saved by our app open correctly in Photopea.
- [ ] A Photopea PSD with text or layer styles, opened and saved by our app,
      still shows that text and those styles in Photopea.
- [ ] Owner edits one real photo end to end and finds it usable.
- [ ] Claude Code, connected over MCP, completes one editing task the owner asks for.

## Decision log

| Date | Change | Why |
| --- | --- | --- |
