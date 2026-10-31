# Prompts

Copy-paste prompts for each kind of agent session. Replace anything in
CAPITALS (like TASK-12 or PR 34). Start each session fresh, in the repo folder.

| File | Use it when | Run in |
| --- | --- | --- |
| `docs/prompts/01-vision.md` | Once, at the start: turn your notes into the vision | Claude Code, Opus |
| `docs/prompts/02-plan.md` | Start of each milestone: catalog, tasks, decisions | Claude Code, Opus |
| `docs/prompts/03-implement.md` | Building one task | Claude Code (Sonnet or Opus) or Antigravity |
| `docs/prompts/04-review.md` | Reviewing one PR (use the other family than the author) | Claude Code or Antigravity |
| `docs/prompts/05-merge.md` | Merging approved PRs and writing the daily status | Claude Code, Opus |
| `docs/prompts/06-batch.md` | Letting Gemini work through a queue while you're away | Antigravity |
| `docs/prompts/07-retro.md` | End of each milestone | Claude Code, Opus |
| `docs/prompts/08-explain.md` | You want to understand something | Either |
| `docs/prompts/09-fix-main.md` | CI on `main` is red, or things feel stuck | Claude Code, Opus |
