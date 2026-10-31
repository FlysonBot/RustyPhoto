# Measured behavior

One file per feature (e.g. `levels.md`), written by the researcher role.
Each file records what a reference app actually outputs, so implementers
match measurements instead of guessing.

Every file must state:

- **Reference:** which app and version, or which Photoshop-saved files.
- **Method:** the exact test inputs (e.g. a 256-step gradient), settings, and how outputs were captured.
- **Result:** the measured values, as a table or a file in `tests/fixtures/` (name it).
- **Confidence:** what was not measured.

Rules: black-box only. Never open any editor's source code, including
Photopea's JavaScript in a browser. Log the source in `docs/provenance-log.md`.
