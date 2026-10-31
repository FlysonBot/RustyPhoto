//! Layer > New > Layer.

use super::{always, CommandSpec, EngineError};
use crate::params::Params;
use editor_doc::Document;
use serde_json::{json, Value};

/// Longest layer name accepted, in characters.
const MAX_NAME: usize = 255;

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec {
        id: "layer.new",
        label: "New Layer",
        params: "name: string = \"Layer N\" — the new layer's name (max 255 chars)",
        enabled: always,
        run: new_layer,
    }]
}

/// Adds a transparent layer above all others and makes it active.
fn new_layer(doc: &mut Document, params: &Params) -> Result<Value, EngineError> {
    let default_name = format!("Layer {}", doc.layers.len());
    let name = params.string("name")?.unwrap_or(&default_name);
    if name.chars().count() > MAX_NAME {
        return Err(EngineError::BadParam {
            name: "name".into(),
            reason: format!("longer than {MAX_NAME} characters"),
        });
    }
    let index = doc.add_layer(name, [0, 0, 0, 0]);
    Ok(json!({ "layer": index }))
}

#[cfg(test)]
mod tests {
    use crate::Engine;
    use editor_doc::Document;
    use serde_json::json;

    #[test]
    fn adds_a_transparent_active_layer() {
        let mut e = Engine::new(Document::new(3, 3, [255; 4]).unwrap());
        e.execute("layer.new", &json!({ "name": "Sky" })).unwrap();
        let doc = e.document();
        assert_eq!(doc.layers.len(), 2);
        assert_eq!(doc.active, 1);
        assert_eq!(doc.layers[1].name, "Sky");
        assert_eq!(doc.layers[1].pixels[0], [0, 0, 0, 0]);
    }

    #[test]
    fn undo_removes_it() {
        let mut e = Engine::new(Document::new(3, 3, [255; 4]).unwrap());
        e.execute("layer.new", &json!({})).unwrap();
        assert!(e.undo());
        assert_eq!(e.document().layers.len(), 1);
    }
}
