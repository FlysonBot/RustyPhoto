# Vision

Status: DRAFT · Last updated: (date) · Owner approval: (not yet)

> The owner fills in the parts marked ✍️ in plain words (15 minutes, no polish
> needed). The architect agent then tidies it up and asks questions. The
> owner approves the final version. Changes after approval are deliberate
> decisions, recorded at the bottom.

## Who it's for ✍️

Who will use this editor, and what are they trying to do?

## What the first usable version must do ✍️

List the things a user must be able to do before you'd call it usable.
Example: open a PNG/JPEG/PSD, work with layers, use common adjustments and
filters, select areas, paint, undo, save.

## What it will NOT do (for now) ✍️

Example: no AI generation features, no web version, no plugins, no CMYK print workflow.

## How it should feel ✍️

Example: familiar to people who use professional editors; fast on large files; dark UI.

## Fixed decisions

- Pure Rust, native desktop app for Linux, Windows and macOS. GUI: egui (`docs/decisions/ADR-001-gui-library.md`).
- Clean room: no other editor's source code is ever read (`AGENTS.md` rule 8).
- Built by agents: Claude Code and Google Antigravity, tasks in Backlog.md.

## Milestones

| Milestone | Means | Gate (checked by) |
| --- | --- | --- |
| M0 Setup | The loop works end to end | One catalog command built, reviewed and merged by agents; CI green on 3 OSes |
| M1 Foundation | A real editor core | Open, layer, edit, undo, save PNG; compositor tests green |
| M2 Breadth | Usable for simple work | Owner-defined list above; common PSDs open correctly |
| M3 Hard features | Layer styles, text, brushes, smart objects | PSD fidelity score target set at M3 start |
| M4 Release | Installers for 3 OSes | Signed builds; owner sign-off |

## Decision log

| Date | Change | Why |
| --- | --- | --- |
