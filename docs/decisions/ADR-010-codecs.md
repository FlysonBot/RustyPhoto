# ADR-010: Image codec libraries

Status: Proposed (2026-10-31, architect)

## Context
M1 needs to open and save PNG; M2 adds JPEG and PSD. Writing PNG and JPEG
codecs ourselves is wasted effort; good permissively licensed Rust crates
exist. PSD needs our own code (no complete, writable PSD crate exists that
keeps unknown blocks byte-for-byte).

## Options
1. **The `image` crate** (MIT/Apache): one API for many formats. Pros: broad
   format support later. Cons: large dependency tree; its in-memory types
   don't match our tiles, so we convert anyway.
2. **Format-specific crates:** `png` (MIT/Apache) now; `zune-jpeg` (MIT/
   Apache/Zlib) for reading and `jpeg-encoder` (MIT/Apache) for writing in
   M2. Pros: small, fast, fuzzed; we control conversion. Cons: one adapter
   per format.
3. **Own codecs.** No.

## Decision
Recommended: **option 2**, inside a new crate `editor-io` (layer L4). PSD
read/write is our own code from Adobe's published PSD file format
specification (allowed by AGENTS.md rule 8; logged in
`docs/provenance-log.md`), in M2.

Every decoder runs with limits: width and height up to
`editor_doc::MAX_DIMENSION`, total decoded size checked before allocating,
and every error returned, never a panic.

## Consequences
- `editor-io` exposes `read(path or bytes) -> Document` and
  `write(&Document, format, options)`, plus a format registry so catalog
  `format.*` entries can become live.
- Each new dependency is added to the workspace with its license noted in
  the PR.
