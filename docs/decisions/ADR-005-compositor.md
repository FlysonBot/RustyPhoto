# ADR-005: Compositor design

Status: Proposed (2026-10-31, architect)

## Context
The compositor turns the layer stack into the picture on screen and in
exported files. It must match the reference app's output (blend modes,
opacity, fill, groups, masks, clipping) and redraw fast enough for a brush
stroke at 60 frames per second on a 6000×4000, 20-layer image. Today a
placeholder in `crates/editor-engine/src/preview.rs` blends every pixel of
every layer with Normal mode, every time.

## Options
1. **CPU, whole image every time** (today). Simple; far too slow at size.
2. **CPU, tile-based with a cache.** Composite 256×256 tiles independently
   (in parallel with `rayon`, MIT/Apache), cache finished tiles, and redraw
   only tiles a command touched. The app uploads only changed tiles to the
   GPU as textures. Pros: deterministic, easy to test pixel-exactly on CI
   machines without a GPU, parallel, plain Rust. Cons: a big filter preview
   on the whole image is slower than a GPU would be.
3. **GPU shaders (wgpu).** Fastest. Cons: results differ between GPU
   drivers, so exact tests are hard; CI has no GPU; harder for agents.

## Decision
Recommended: **option 2**, in a new crate `editor-render` (layer L3).

- Input: the document's layer tree. Output: straight RGBA8 for a requested
  rectangle. A full composite is "every tile".
- Per tile: convert to premultiplied `f32` (ADR-002), walk the tree bottom
  to top, apply each layer's mask, blend mode, fill then opacity, clip
  groups; groups are "pass through" or isolated as in PSD.
- Blend formulas: W3C *Compositing and Blending Level 1* for the modes it
  defines; measured black-box behavior (`docs/behavior/`) for the rest.
- Cache key per tile: the tile pointers of every layer under it plus the
  layer properties. Unchanged pointers mean the cached tile is still right.
- GPU compositing may come later behind the same API, tested against the
  CPU version.

## Consequences
- `crates/editor-engine/src/preview.rs` is removed when `editor-render` lands.
- Golden tests: small fixture stacks with exact expected pixels, plus
  tolerance tests against PSD embedded composites (M2).
- Adjustment layers (M2) are compositor steps, not pixel layers.
