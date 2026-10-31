# Image Editor

An AI-powered Photopea-like local image editor written in Rust, aiming
to empower users to burnish their rusty photo into polished ones. Work
on Linux, Windows, and macOS.

## Run it

```
cargo run -p editor-app
```

The window shows every planned feature in the menus. Ones that don't exist
yet are greyed out; hover to see their id. The status bar counts how many
are live.

## Check it

```
cargo xtask ci
```

This runs every check CI runs: formatting, strict lints, the layer rules, the
catalog coverage, doc paths and all tests. It must pass before any merge.

## Where to start

- Agents: read `AGENTS.md`.
- People: read `docs/vision.md`, then `docs/architecture.md`.
- Progress: `docs/generated/coverage.md` and `docs/status.md`.
- Work queue: `backlog board` (needs Backlog.md: `npm i -g backlog.md`).
