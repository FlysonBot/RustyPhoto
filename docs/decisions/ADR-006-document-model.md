# ADR-006: Document model

Status: Proposed (2026-10-31, architect)

## Context
Today a document is a flat list of canvas-sized layers, each a `Vec` of
pixels. M2 needs: PSD layers that sit partly outside the canvas, groups,
masks, adjustment layers, a selection, and keeping PSD content we don't
understand (text, smart objects, styles) byte-for-byte. ADR-004 needs tiles.

## Options
1. **Keep flat canvas-sized buffers.** Simple; no off-canvas pixels (PSD
   loses data), no cheap undo, memory scales with canvas size even for an
   empty layer.
2. **Each layer one buffer with its own bounds** (like PSD files store it).
   Fits PSD; but undo still copies whole layers.
3. **Sparse tile grid per layer.** A layer is a map from tile position
   (signed, so it can extend past the canvas edge) to a shared 256×256 tile.
   Missing tiles are fully transparent and cost nothing.

## Decision
Recommended: **option 3**, built in this order:

1. **Tiles** (`editor-doc`): `TiledBuffer` with 256×256 RGBA8 tiles in
   `Arc`, copy-on-write, sparse, signed tile coordinates. Pixel access by
   API only (`get`, `set`, rectangle read/write, tile iteration).
2. **Layers** keep `name`, `visible`, `opacity`; gain `blend_mode`, `fill`,
   `locks`, and a `kind`: Pixel now; Group, Adjustment, Fill and
   Unsupported (opaque PSD data shown from its stored pixels) in M2.
3. **Layer tree**: the document holds an ordered list where a Group layer
   has children. Layers are addressed by a stable `LayerId` (a number that
   never changes), not by position, so agents and history stay correct when
   layers move. Commands accept either an id or, for convenience, an index.
4. **Selection** (M2): an 8-bit mask `TiledBuffer`, part of the document,
   so selection changes are undoable steps.
5. **Unknown data**: each layer and the document carry an `extra` byte blob
   map (for example PSD "additional layer info" blocks we don't parse) that
   save writes back unchanged when the layer wasn't edited.

## Consequences
- Every command reaches pixels through the tile API (also ADR-003).
- `Document.layers: Vec<Layer>` and `active: usize` change; the migration
  task updates every existing command in one PR.
- Off-canvas pixels survive crops and moves, as users expect.
