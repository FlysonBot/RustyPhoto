# ADR-002: Pixel format and alpha

Status: Proposed (2026-10-31, architect)

## Context
Every pixel in the document needs one agreed layout. The choice decides how
exact PSD and PNG round trips are, how fast compositing is, and how much
memory a 6000×4000 image with 20 layers takes. Today pixels are `Rgba8`
(`[u8; 4]`), straight alpha, in sRGB.

"Straight" alpha keeps the color as the user picked it, even where the pixel
is almost transparent. "Premultiplied" stores color already multiplied by
alpha (like storing "how much red light", not "what red").

## Options
1. **Straight RGBA8 storage, premultiplied float math inside the compositor.**
   Pros: PSD and PNG store straight 8-bit, so open/save is lossless and
   byte-exact; color under transparent pixels survives (users rely on that
   when they erase and then restore). Compositing converts to premultiplied
   `f32` per tile, where the blend math is simplest and correct.
   Cons: one conversion per tile per composite (cheap next to the blend math).
2. **Premultiplied RGBA8 storage.** Pros: fast compositing, no conversion.
   Cons: loses color at low alpha (at alpha 1/255 every color rounds to a
   few values); PSD round trip is no longer byte-exact. Breaks the vision's
   "content the user didn't touch is written back unchanged".
3. **Premultiplied f32 storage.** Pros: exact math. Cons: 4× memory
   (6000×4000×20 layers ≈ 7.7 GB), too much for the 2 GB history budget.

## Decision
Recommended: **option 1**. Document pixels stay straight-alpha RGBA, 8 bits
per channel, sRGB-encoded. The compositor (ADR-005) converts to
premultiplied `f32` internally and back at the end, rounding to nearest.
Blending happens on the sRGB-encoded values (not linear light), because
that is what the reference app and PSD composites do; matching their output
is the goal. Linear-light blending can be an option later.

Color management: M1/M2 assume sRGB. An embedded ICC profile is kept as
opaque bytes and written back unchanged (ADR-006), not applied.

## Consequences
- `editor-doc` keeps `Rgba8 = [u8; 4]`, straight alpha.
- `editor-render` owns the only straight↔premultiplied conversion; tests
  check that converting and back is exact for every alpha 1..=255.
- Fully transparent pixels keep their color bytes; no command may zero them
  unless that is its job (eraser zeroes alpha only).
