//! The feature catalog: every feature the finished editor should have,
//! each with a stable id. The file itself lives at `catalog/catalog.toml`
//! in the repository root and is compiled into this crate.
//!
//! An entry is "live" when a command with the same id exists in the engine.
//! `cargo xtask catalog` computes coverage from that.

use serde::Deserialize;

/// The raw catalog file, embedded at build time.
pub const CATALOG_TOML: &str = include_str!("../../../catalog/catalog.toml");

/// One target feature.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// Stable id, e.g. `filter.blur.gaussian`. Never change one once used.
    pub id: String,
    /// Menu path, e.g. `Filter/Blur`. Empty for tools and formats.
    #[serde(default)]
    pub menu: String,
    /// Text shown to the user.
    pub label: String,
    /// Keyboard shortcut in the form `Ctrl+Shift+N`, if any.
    #[serde(default)]
    pub shortcut: Option<String>,
    /// What kind of feature: `command` (default), `tool` or `format`.
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_kind() -> String {
    "command".to_string()
}

#[derive(Debug, Deserialize)]
struct CatalogFile {
    #[serde(default, rename = "item")]
    items: Vec<Entry>,
}

/// Parses the embedded catalog.
pub fn load() -> Result<Vec<Entry>, String> {
    parse(CATALOG_TOML)
}

/// Parses catalog text. Fails on syntax errors and duplicate ids.
pub fn parse(text: &str) -> Result<Vec<Entry>, String> {
    let file: CatalogFile =
        toml::from_str(text).map_err(|e| format!("catalog/catalog.toml: {e}"))?;
    let mut seen = std::collections::BTreeSet::new();
    for entry in &file.items {
        if !seen.insert(entry.id.as_str()) {
            return Err(format!("catalog/catalog.toml: duplicate id `{}`", entry.id));
        }
    }
    Ok(file.items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_parses() {
        let entries = load().unwrap();
        assert!(!entries.is_empty());
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let text = "[[item]]\nid='a'\nlabel='A'\n[[item]]\nid='a'\nlabel='B'\n";
        assert!(parse(text).is_err());
    }
}
