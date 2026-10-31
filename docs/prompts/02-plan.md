# Prompt: plan a milestone (architect)

Run at the start of each milestone. For the first run, MILESTONE is M0+M1.

---

You are the architect. Read AGENTS.md, docs/vision.md (approved),
docs/architecture.md, docs/gaps.md and docs/generated/coverage.md.
Run `backlog instructions overview` to learn the task tool.

Plan MILESTONE:

1. **Catalog (first run only):** expand catalog/catalog.toml to the full
   target product the vision describes: every menu item, tool and file
   format of a professional image editor, grouped by menu, with stable ids
   and conventional shortcuts. Expect several hundred entries. Never use
   another product's name in labels. Run `cargo xtask catalog`.
2. **Decisions:** list the architecture decisions this milestone needs
   (for M1 at least: pixel format and alpha, bit depth, undo storage,
   compositor design). Write each as an ADR in docs/decisions/ with status
   Proposed, options and your recommendation.
3. **Tasks:** create Backlog.md tasks for the milestone. Each task:
   - small: a few hours of agent work, a PR under ~500 lines
   - label for the track (`track:catalog` or `track:design`) and the agent
     (`agent:opus` for hard core work, `agent:sonnet` or `agent:gemini` for
     well-specified work, `agent:gemini-batch` for chores)
   - acceptance criteria with numbers, "Files allowed", and dependencies
   - no two tasks that can run at the same time may share a file
   Start with docs/gaps.md G1 and G2 (agents need eyes and hands first).
4. Update docs/gaps.md and docs/architecture.md to match.
5. Run `cargo xtask ci`, commit on a branch `plan-MILESTONE`, open a PR.
6. Tell me: the decisions I must make (each with your recommendation), the
   task list in one line each, and which tasks can start in parallel today.
