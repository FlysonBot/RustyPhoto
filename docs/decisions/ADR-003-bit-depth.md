# ADR-003: Bit depth

Status: Proposed (2026-10-31, architect)

## Context
The vision rules out 16-bit color and CMYK for M2, but a professional editor
will need 16-bit (and 32-bit float for HDR) later. If 8-bit is baked into
every command now, adding 16-bit later means rewriting all of them.

## Options
1. **8-bit only, hard-coded everywhere.** Pros: simplest. Cons: a 16-bit
   later costs a rewrite of every command and the compositor.
2. **Generic over a channel type from day one** (`Buffer<T>` with `u8`,
   `u16`, `f32`). Pros: ready for 16/32-bit. Cons: every command becomes
   generic code; much harder for agents to write and review; slows M1–M2
   for a feature nobody needs yet.
3. **8-bit storage behind one seam.** Pixels are reached only through the
   tile API (ADR-006), never as raw slices outside `editor-doc`; pixel math
   in `editor-algo` takes and returns normalized `f32` per channel where
   that is natural. Adding a 16-bit tile type later touches the tile module,
   the compositor and file I/O, not every command.
   Cons: some discipline in reviews; a small cost converting to `f32`.

## Decision
Recommended: **option 3**. M1 and M2 are 8 bits per channel. The tile type
has one variant now. `image.mode.depth16` and `image.mode.depth32` stay
catalog entries for later.

16-bit PNG and PSD files open by converting to 8-bit with
`round(v * 255 / 65535)`, and the app tells the user it did so.

## Consequences
- Reviewers reject new code that indexes raw pixel `Vec`s outside
  `editor-doc` and `editor-render`.
- A future 16-bit ADR replaces this one.
