# ADR-004: History (undo) storage

Status: Proposed (2026-10-31, architect)

## Context
The vision asks for unlimited history steps inside a memory budget
(default 2 GB, adjustable), dropping the oldest steps with a notice, a
History panel with timestamps, and undo under 100 ms on a 6000×4000 image
with 20 layers. Today every command clones the whole document and keeps up
to 50 clones: one 6000×4000 layer is 96 MB, so 20 layers × 50 steps would be
96 GB.

## Options
1. **Whole-document snapshots** (today). Pros: trivially correct. Cons:
   impossible memory use at the target size.
2. **Command-specific inverse operations** ("undo by running the reverse").
   Pros: tiny memory. Cons: every command needs an exact inverse, written
   and tested by hand; many have none (blur); one bug corrupts history.
3. **Copy-on-write tiles, snapshots share unchanged tiles.** Pixels live in
   256×256 tiles behind a shared pointer (`Arc`, like a reference-counted
   object in Java or Python). Cloning a document copies only pointers; a
   command that changes one pixel copies one tile (256 KB). Each history
   step is still "the whole document", so undo is a pointer swap and always
   exact, but it costs only the tiles that step changed.
   Cons: memory accounting must count each shared tile once.

## Decision
Recommended: **option 3**, together with ADR-006's tiled layers.

- History is a list of entries `{ label, timestamp, document snapshot }`.
  Undo/redo swap snapshots (constant time, well under 100 ms).
- Memory used = bytes of all *distinct* tiles reachable from history but not
  from the current document. When it exceeds the budget, the oldest entries
  are dropped until it fits, and the engine reports how many it dropped (the
  app shows the notice).
- Budget default 2 GiB, set in Performance preferences, never below 64 MiB.
- Commands keep today's contract: run on a copy, success records exactly one
  step, failure changes nothing. Session-only commands (ADR-009) record none.
- Closing or replacing the document clears history (vision).

## Consequences
- Command authors write no undo code at all; the fuzz test and per-command
  "undo restores the exact document" tests keep working unchanged.
- A command that rewrites every pixel of every layer costs a full copy in
  history. That's correct, and the budget handles it.
- History branches (a later version) fit: snapshots are already immutable.
