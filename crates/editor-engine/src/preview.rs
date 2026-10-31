//! A naive compositor so the app has something to show.
//!
//! PLACEHOLDER: Normal blend only, 8-bit, no masks, no groups, no tiling.
//! A real compositor belongs in its own render crate (see docs/gaps.md).

use editor_doc::{Document, Rgba8};

/// Flattens all visible layers with Normal blending over transparency.
pub fn flatten(doc: &Document) -> Vec<Rgba8> {
    let mut out: Vec<[f32; 4]> = vec![[0.0; 4]; doc.pixel_count()];
    for layer in doc.layers.iter().filter(|l| l.visible) {
        let opacity = layer.opacity.clamp(0.0, 1.0);
        for (dst, src) in out.iter_mut().zip(&layer.pixels) {
            let sa = f32::from(src[3]) / 255.0 * opacity;
            let da = dst[3];
            let oa = sa + da * (1.0 - sa);
            if oa > 0.0 {
                for c in 0..3 {
                    let s = f32::from(src.get(c).copied().unwrap_or(0)) / 255.0;
                    let d = dst.get(c).copied().unwrap_or(0.0);
                    let v = (s * sa + d * da * (1.0 - sa)) / oa;
                    if let Some(slot) = dst.get_mut(c) {
                        *slot = v;
                    }
                }
            }
            dst[3] = oa;
        }
    }
    out.iter()
        .map(|p| p.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_top_layer_wins() {
        let mut doc = Document::new(1, 1, [255, 0, 0, 255]).unwrap();
        doc.add_layer("top", [0, 0, 255, 255]);
        assert_eq!(flatten(&doc), vec![[0, 0, 255, 255]]);
    }

    #[test]
    fn hidden_layers_are_skipped() {
        let mut doc = Document::new(1, 1, [255, 0, 0, 255]).unwrap();
        doc.add_layer("top", [0, 0, 255, 255]);
        doc.layers[1].visible = false;
        assert_eq!(flatten(&doc), vec![[255, 0, 0, 255]]);
    }
}
