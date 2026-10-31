# Glossary

Use these words exactly. Add a term the moment two people (or agents) use a
word differently.

| Term | Meaning here |
| --- | --- |
| Catalog | `catalog/catalog.toml`: the full list of target features, each with a stable id |
| Catalog id | A feature's permanent name, e.g. `filter.blur.gaussian`. Never renamed |
| Catalog kind | What an entry is: `command` (menu item), `tool` (toolbar), `panel` (panel button or panel menu), `key` (keyboard only), `format` (file format) |
| Live | A catalog entry that has a registered command with the same id |
| Command | One user action, registered as a `CommandSpec`; runs through `Engine::execute` and records one undo step |
| Gate | `cargo xtask ci`: the checks every change must pass |
| Floor | A number that may only go up (`catalog/floor.txt`); lowering it needs owner approval |
| Layer (code) | A level in the crate hierarchy, L0–L6. Not the same as an image layer |
| Image layer | One bitmap in the document's layer stack |
| Opacity | How transparent a whole image layer is, including its effects |
| Fill | Like opacity, but excludes layer styles (planned) |
| Straight alpha | Color stored independently of alpha (current pixel format) |
| Premultiplied alpha | Color already multiplied by alpha (a pending decision) |
| Oracle | A reference whose output we compare against: Photoshop-saved composites inside PSD files, or Photopea used as a black box |
| Catalog track | Work that fills a catalog entry with the standard recipe; no spec approval needed |
| Design track | Work that needs a spec approved by the owner: new architecture, contracts, ambiguous behavior |
| Cross-family review | Claude reviews Gemini's PRs and Gemini reviews Claude's |
| Handoff note | What's done, what's left and what's odd, written in the task before an agent stops |
