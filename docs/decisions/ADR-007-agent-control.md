# ADR-007: Agent control channel

Status: Accepted (2026-10-31, by owner)

## Context
Agents building this app need to drive it and inspect results without a
person (gap G2). The vision's M2 gate needs Claude Code to control the
running app over MCP (Model Context Protocol): run any command, read the
document state as data, get a rendered PNG, paint strokes.

## Options
1. **MCP server inside the GUI app first.** Matches the end goal, but needs
   a running window, threads and a protocol library before anything works;
   hard to test in CI.
2. **Headless CLI first, MCP later, both on one engine API.** A command-line
   program `editor-cli` loads or creates a document, runs a list of
   commands (JSON), prints the state as JSON and writes a rendered PNG. No
   window, so CI and agents can use it at once. In M2 an MCP server exposes
   the same three operations (run, state, render) from the app.
3. **A scripting language (Lua, Python) embedded.** Powerful; a large new
   surface; not needed for agent control.

## Decision
Recommended: **option 2**.

- `editor-engine` gets one stable, documented state format
  (a new spec, docs/specs/state-json.md): document size, layers (id, name, kind,
  visible, opacity, blend mode), active layer, history entries, and the
  command list with parameter specs and enabled flags.
- `editor-cli` (new crate, L6 but no GUI) calls only public engine and I/O
  APIs, so whatever the CLI can do, MCP can later do the same way.
- The MCP server (M2) is a separate task and ADR for transport details.

## Consequences
- The state JSON format is a contract: changing it needs a spec update.
- End-to-end tests (the M1 gate) can be written as CLI scripts.
