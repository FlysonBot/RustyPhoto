# AGENTS.md: read this first

Every agent reads this file before doing anything: Claude, Gemini, any other.

This project is a native image editor written in pure Rust for Linux, Windows
and macOS. The GUI is egui. **Never** add a web view, Tauri, Electron or any
JavaScript UI. **Never** open, copy or translate the source code of Photopea,
Photoshop or any other editor (clean room: see rule 8). These limits outrank
everything below.

The owner (a person) does not read Rust. Explain your work in plain English;
compare with Python or Java when that helps.

## 1. Orientation (read in this order, 5 minutes)

| File | Why |
| --- | --- |
| `docs/vision.md` | What we build, for whom, and what we don't |
| `docs/architecture.md` | Crates, layers, how a command flows |
| `docs/glossary.md` | Exact meaning of terms. Use these words |
| `docs/generated/coverage.md` | What exists, measured (generated, never edit) |
| `docs/gaps.md` | Ranked list of what's missing |
| Your Backlog.md task (`backlog task view <id>`) | What to do now |
| `docs/recipes/add-a-command.md` | How to add any feature |

## 2. Code map

| Crate | Layer | Holds |
| --- | --- | --- |
| `crates/editor-doc` | L0 | Document model: layers, pixels |
| `crates/editor-catalog` | L0 | The feature catalog as data (`catalog/catalog.toml`) |
| `crates/editor-engine` | L5 | Command registry, undo/redo, all commands (`crates/editor-engine/src/cmd`) |
| `crates/editor-app` | L6 | The desktop app: window, menus, canvas |
| `xtask` | tool | Project checks (`cargo xtask ...`) |

A crate may depend only on lower layers. Only L6 may use egui or other GUI
crates. The table lives in `xtask/layers.toml`; a new crate needs a line there.

## 3. Golden rules

Each rule says what to do instead and how it is checked.

1. **Never crash.** No `unwrap`, `expect`, `panic!`, `todo!`, `unreachable!`
   or `[i]` indexing in product code. Return the crate's error type; use
   `.get(i)`. Tests may unwrap. *Checked by:* workspace clippy lints and the
   fuzz test `crates/editor-engine/tests/panic_hunt.rs`. Never crash outranks feature work.
2. **Every feature is a command** with an id from `catalog/catalog.toml`.
   Using the catalog id makes the menu item live with no UI work.
   *Checked by:* `cargo xtask catalog` (fails on ids not in the catalog).
3. **Depend only downward.** *Checked by:* `cargo xtask layers`.
4. **No unsafe code.** *Checked by:* `unsafe_code = "forbid"`.
5. **Never weaken a check to pass it.** Don't edit expected test outputs,
   lower `catalog/floor.txt`, delete tests or add `#[allow]` without the
   owner's approval in the task. If a check is wrong, say so in the task.
6. **Look at UI changes.** Take a snapshot, look at it, and attach it to
   the PR (`docs/recipes/check-ui.md`). *Checked by:* snapshot tests in
   `crates/editor-app/tests/snapshot.rs`, run by CI (Linux on every PR;
   Windows and macOS after merge to main).
7. **No OS-specific code** outside a future `platform` crate.
8. **Clean room.** Learn behavior only from outputs, published specs, papers
   and permissively licensed code (MIT, BSD, Apache). Never GPL code. Log
   every outside source in `docs/provenance-log.md`.
9. **Don't change CI** (`.github/workflows/ci.yml`) in feature PRs.
10. **Product names** of other software never appear in the app's UI text.

## 4. Picking work

Look with `git fetch` then `backlog board`: the board includes other
agents' branches. `backlog task list` shows only your own checkout, so it
misses other agents' claims.

1. A Backlog.md task with your agent label, status To Do or In Progress.
2. Else the highest-priority To Do task nobody has claimed. Claim it before
   starting: in your worktree, assign yourself, set In Progress, then commit
   and push that change before any other work. Until it is committed, no
   other agent can see the claim. If two claims show up, the later one gives way.
3. Else the top entry in `docs/gaps.md` nobody holds: create a task for it,
   then commit and push it the same way.

Product questions (what should the user experience?): write the question in
the task, set it Blocked, and pick other work. Technical choices inside your
task's files are yours.

Before coding, write your plan into the task. Design-track tasks wait for
plan approval; catalog-track tasks proceed.

## 5. Before you finish (run each that applies)

| When | Run |
| --- | --- |
| Always | `cargo xtask ci` (the exact gate CI runs) |
| You added a command | `cargo xtask catalog`, commit `docs/generated/coverage.md`, raise `catalog/floor.txt` to the new live count |
| You moved a file | Fix every doc that mentions it (`cargo xtask docpaths` finds them) |
| You closed part of a gap | Update its entry in `docs/gaps.md` (evidence, date) |
| Always | Write the task's final summary; open a PR with the template |

"Done" means: command + UI + tests, gate green. Never claim done otherwise.

## 6. Parallel agents

- Use your own build folder, the same one for all your worktrees (so libraries
  aren't recompiled per branch; never share it with another agent):
  `export CARGO_TARGET_DIR="$(git rev-parse --path-format=absolute --git-common-dir)/../target/agent-<name>"`
  (PowerShell: `$env:CARGO_TARGET_DIR = "$(git rev-parse --path-format=absolute --git-common-dir)/../target/agent-<name>"`).
  That is `<main checkout>/target/agent-<name>`, with your agent name in place of `<name>`.
- Folder doesn't exist yet? Copy another agent's instead of compiling from
  scratch (about 20 s instead of about 4 min). Keep file timestamps, or Cargo
  rebuilds everything; copy only while that agent isn't building:
  `cp -a --reflink=auto <main checkout>/target/agent-<other> <main checkout>/target/agent-<name>`
  (Windows: `robocopy <from> <to> /E /DCOPY:T`, which keeps timestamps).
- Work in your own git worktree and branch: `t<task-number>-<short-name>`.
  Create it before you change any task: backlog commands that change a task
  (claim, edit, create) write into the checkout they run in, so run them
  inside your worktree, never in the main checkout. Reading tasks works anywhere.
- New feature = new file. Never grow a shared file when a new module works.
- Hot spots (re-read right before editing; keep edits to one line where possible):
  `crates/editor-engine/src/cmd/mod.rs` (the `command_specs` list), `catalog/catalog.toml`, `catalog/floor.txt`.
- If someone else's change breaks the build, wait and retry; don't fix their files.
- Keep the tree building at every step. If you hit a usage limit or must stop,
  write a handoff note in the task first: what's done, what's left, what's odd.
- Two failed attempts at the same problem: stop, write it up in the task, mark it Blocked.

## 7. Where things are tracked (one home per fact)

| Fact | Home |
| --- | --- |
| Tasks, plans, handoffs | Backlog.md (`backlog/`) |
| What exists | `docs/generated/coverage.md` (generated) |
| What's missing, ranked | `docs/gaps.md` |
| Why we chose X | `docs/decisions/` |
| Measured behavior of reference apps | `docs/behavior/` |
| Outside sources consulted | `docs/provenance-log.md` |
| Daily status | `docs/status.md` (links to numbers, never copies them) |

## 8. Delegations from the owner

- Catalog-track PRs may merge after cross-family review and a green gate,
  without waiting for the owner. Design-track PRs wait for the owner.
- (The owner adds more here over time.)

<!-- BACKLOG.MD GUIDELINES START -->
<!-- backlog.md-instructions-version: 1.55.3 -->
<CRITICAL_INSTRUCTION>

## Backlog.md Workflow

This project uses Backlog.md for task and project management.

**At the beginning of each conversation in this project, run `backlog instructions overview` before answering or taking action. Re-read it only if you have not read it yet in the current conversation.**

Use the overview to decide whether to search, read, create, or update Backlog tasks.

Before task lifecycle actions, read the matching detailed guide:
- `backlog instructions task-creation` before creating or splitting tasks
- `backlog instructions task-execution` before planning, changing status or assignee, adding a plan or implementation notes, or implementing task work
- `backlog instructions task-finalization` before checking acceptance criteria, writing final summaries, or moving tasks to terminal statuses

Use `backlog <command> --help` before running unfamiliar commands. Help shows options, fields, and examples.

Do not edit Backlog task, draft, document, decision, or milestone markdown files directly. Use the `backlog` CLI so metadata, relationships, and history stay consistent.

</CRITICAL_INSTRUCTION>
<!-- BACKLOG.MD GUIDELINES END -->
