# ADR-009: Command effects and session state

Status: Accepted (2026-10-31, by owner)

## Context
Every command today edits the document and records one undo step. But many
catalog entries don't edit the document: zoom, choosing colors, picking a
tool, export (reads only), open and new (replace the document and clear
history). The catalog check counts a feature as live only if it is an engine
command, and agents must be able to do everything a user can, so these must
be commands too.

## Options
1. **Keep one kind; app-only features live outside the engine.** Then zoom,
   colors and tools can never be "live" or reachable over MCP.
2. **An `effect` field on every command.** The engine handles each kind:
   - `Edit`: runs on a copy of the document, records one history step.
   - `Session`: changes session state that is not part of the image and not
     undoable (view zoom/pan, foreground/background color, current tool and
     its options, preferences).
   - `ReadOnly`: reads the document, writes something outside (export).
   - `Replace`: produces a new document (new, open); history is cleared.
3. **Separate registries per kind.** More types, same outcome as 2.

## Decision
Recommended: **option 2**. The engine owns a `Session` value next to the
document and history. `RunFn` receives a context giving access to what its
effect allows (an `Edit` command can't touch the view; a `Session` command
can't touch pixels).

Which layer is active and which layers are selected live in the `Session`,
not in the document, so selecting a layer is a `Session` command and never a
history step. Layers are referred to by their permanent `LayerId`
(ADR-006). After undo, redo or any command, the engine checks that the active
layer still exists; if not, the topmost remaining layer becomes active. An
`Edit` command reads the active layer from the session but cannot change it,
except through its result (for example, Layer > New makes the new layer
active).

## Consequences
- `CommandSpec` and `RunFn` change once; every existing command is updated
  in that PR.
- The state JSON (ADR-007) includes the session.
- Tool strokes (brush, eraser) are `Edit` commands that take a list of
  points, which is what MCP sends.
