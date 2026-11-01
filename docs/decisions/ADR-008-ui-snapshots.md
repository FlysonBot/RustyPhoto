# ADR-008: UI snapshots for agents

Status: Accepted (2026-10-31, by owner)

## Context
Agents can't see the app (gap G1), so today every UI change waits for the
owner to look (AGENTS.md rule 6). Agents need to render the app to a PNG
without a window and look at it themselves.

## Options
1. **`egui_kittest`** (MIT/Apache, from the egui authors, same version as
   egui 0.37): runs the app's UI in a test harness, simulates clicks and
   keys, and renders frames to an image with `wgpu`. Pros: official, also
   gives UI interaction tests. Cons: rendering needs a GPU adapter or a
   software one; GitHub's Linux runners may have neither.
2. **Screenshot a real window** (OS tools). OS-specific, flaky, needs a
   desktop session.
3. **Our own software rasterizer of egui's output.** No GPU needed; real
   work to write and maintain.

## Decision
Recommended: **option 1**.

- Move the app's state and UI into a library part of `editor-app` so tests
  and a `--snapshot out.png` flag can drive it without a window.
- `cargo run -p editor-app -- --snapshot out.png` renders one frame (size
  set by `--size WxH`) and exits.
- Snapshot tests compare against stored PNGs with a small tolerance. Where
  no GPU adapter exists they skip with a printed notice instead of failing,
  so CI (rule 9: we don't change CI in feature PRs) stays green; agents run
  them locally. Enabling software rendering on CI is a separate CI change
  for the owner to approve.

## Consequences
- AGENTS.md rule 6 changes to "attach the snapshot PNG and look at it".
- Interaction tests (click a menu, check the status bar) become possible.
