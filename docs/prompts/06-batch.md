# Prompt: batch queue (Gemini in Antigravity)

Start this when you step away. Your computer must stay on and awake.

---

Read AGENTS.md. Your agent name is gemini-batch.

Work through the queue: every Backlog.md task labeled `agent:gemini-batch`
with status To Do, highest priority first. For each task, follow
docs/prompts/03-implement.md exactly, one task per branch and PR.

Rules for unattended work:
- Never merge anything. Never touch main directly.
- If a task is unclear or needs a product decision, mark it Blocked with
  your question and take the next one.
- If `cargo xtask ci` fails twice for the same reason, write a handoff note,
  mark the task Blocked and move on.
- When the queue is empty or your usage limit is near, stop and write a
  summary in a new task titled "Batch report DATE": tasks done (with PR
  links), tasks blocked (with the reason).
