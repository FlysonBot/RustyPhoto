# Provenance log

Append-only. One line per outside source consulted for behavior, formats or
algorithms. This is the project's clean-room record: it shows where every
piece of knowledge came from.

Allowed: observed outputs of reference apps, published specifications,
research papers, permissively licensed code (MIT, BSD, Apache; note the
license). Never: any editor's own source code, GPL code.

| Date | Agent | Source (link or description) | License / kind | Used for |
| --- | --- | --- | --- | --- |
| (date) | setup | egui / eframe documentation | MIT OR Apache-2.0 | App shell |
| 2026-10-31 | architect (Claude Opus) | General knowledge of conventional image-editor menus, tool names and shortcuts, from user-facing behavior and documentation; no source code read | Observed behavior | `catalog/catalog.toml` outline |
| 2026-10-31 | architect (Claude Opus) | W3C Compositing and Blending Level 1 (https://www.w3.org/TR/compositing-1/) | Published spec | ADR-005 blend formulas |
