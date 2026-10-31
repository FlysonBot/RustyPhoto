//! Image > Adjustments > Invert. A reference example of a complete command.

use super::{has_layers, CommandSpec, EngineError};
use crate::params::Params;
use editor_doc::Document;
use serde_json::{json, Value};

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec {
        id: "image.adjust.invert",
        label: "Invert",
        params: "layer: index = active layer — the layer to invert",
        enabled: has_layers,
        run: invert,
    }]
}

/// Replaces each color channel c with 255 - c. Alpha is unchanged.
fn invert(doc: &mut Document, params: &Params) -> Result<Value, EngineError> {
    let index = params.layer(doc.active)?;
    let layer = doc.layer_mut(index)?;
    for px in &mut layer.pixels {
        let [r, g, b, a] = *px;
        *px = [255 - r, 255 - g, 255 - b, a];
    }
    Ok(json!({ "layer": index }))
}

#[cfg(test)]
mod tests {
    use crate::Engine;
    use editor_doc::Document;
    use serde_json::json;

    fn engine() -> Engine {
        Engine::new(Document::new(2, 2, [10, 20, 30, 200]).unwrap())
    }

    #[test]
    fn inverts_color_and_keeps_alpha() {
        let mut e = engine();
        e.execute("image.adjust.invert", &json!({})).unwrap();
        assert_eq!(e.document().layers[0].pixels[0], [245, 235, 225, 200]);
    }

    #[test]
    fn inverting_twice_restores_the_original() {
        let mut e = engine();
        let before = e.document().clone();
        e.execute("image.adjust.invert", &json!({})).unwrap();
        e.execute("image.adjust.invert", &json!({})).unwrap();
        assert_eq!(e.document(), &before);
    }

    #[test]
    fn undo_restores_exact_pixels() {
        let mut e = engine();
        let before = e.document().clone();
        e.execute("image.adjust.invert", &json!({})).unwrap();
        assert!(e.undo());
        assert_eq!(e.document(), &before);
        assert!(e.redo());
        assert_eq!(e.document().layers[0].pixels[0], [245, 235, 225, 200]);
    }

    #[test]
    fn missing_layer_is_an_error_and_changes_nothing() {
        let mut e = engine();
        let before = e.document().clone();
        assert!(e
            .execute("image.adjust.invert", &json!({ "layer": 9 }))
            .is_err());
        assert_eq!(e.document(), &before);
        assert!(!e.can_undo());
    }
}
