# Prompt: implement one task

Use one fresh session per task. Replace TASK-ID and NAME (e.g. sonnet-1).

---

Read AGENTS.md. Your agent name is NAME. Work on TASK-ID only.

1. Create a git worktree and branch for the task, and use the build folder
   `<main checkout>/target/agent-NAME`, shared by all your worktrees; if it
   doesn't exist yet, copy another agent's first (all: AGENTS.md section 6).
   Run every later step from inside this worktree.
2. Read the task with the backlog CLI, and everything it links to.
   Assign it to yourself and set it In Progress.
3. Write your implementation plan into the task. If the task is
   track:design, stop and wait for my approval of the plan. Otherwise go on.
4. Implement it, touching only the task's "Files allowed". If you need
   another file, stop and explain why in the task.
   For a new command, follow docs/recipes/add-a-command.md exactly.
5. Run `cargo xtask ci` until it passes. Never weaken a check (rule 5).
6. Tick the acceptance criteria you met, write the final summary in the
   task, push the branch and open a PR with the template. Fill in
   "I'm not sure about" honestly.
7. Reply to me in plain English, 5 lines max: what you built, how it was
   verified, the PR link, and anything I should look at.

If you hit a usage limit or must stop early, first write a handoff note in
the task (done / left / odd) and push your branch.
