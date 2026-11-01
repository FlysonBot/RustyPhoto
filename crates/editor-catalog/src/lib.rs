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
    /// What kind of feature: `command` (default, a menu item), `tool`
    /// (toolbar), `panel` (panel button or panel menu), `key` (keyboard only)
    /// or `format` (file format).
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

    /// AGENTS.md rule 10: other products' names never appear in UI text.
    #[test]
    fn labels_never_name_other_products() {
        const BANNED: &[&str] = &[
            "photoshop",
            "photopea",
            "adobe",
            "gimp",
            "krita",
            "affinity",
            "lightroom",
            "pixelmator",
            "paint.net",
            "corel",
        ];
        for e in load().unwrap() {
            // Both are shown in the UI: the label, and the menu path.
            let shown = format!("{} {}", e.label, e.menu).to_lowercase();
            for name in BANNED {
                assert!(
                    !shown.contains(name),
                    "`{}` label or menu names `{name}`",
                    e.id
                );
            }
        }
    }

    /// Every kind is known; commands have a menu path, other kinds don't;
    /// ids are lowercase dotted words.
    #[test]
    fn entries_are_well_formed() {
        for e in load().unwrap() {
            assert!(
                ["command", "tool", "panel", "key", "format"].contains(&e.kind.as_str()),
                "`{}` has unknown kind `{}`",
                e.id,
                e.kind
            );
            assert_eq!(
                e.kind == "command",
                !e.menu.is_empty(),
                "`{}`: commands need a menu, other kinds must not have one",
                e.id
            );
            let ok = e.id.split('.').count() >= 2
                && e.id.split('.').all(|part| {
                    !part.is_empty()
                        && part
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                });
            assert!(ok, "id `{}` is not lowercase dotted words", e.id);
        }
    }

    /// The same shortcut however it is written: modifiers sorted, lowercase.
    /// `Shift+Ctrl+X` and `Ctrl+Shift+X` both become `ctrl+shift+x`.
    fn normalize_shortcut(s: &str) -> String {
        let lower = s.to_lowercase();
        let parts: Vec<&str> = lower.split('+').collect();
        let (key, modifiers) = match parts.split_last() {
            Some((key, modifiers)) => (*key, modifiers),
            None => return lower,
        };
        let mut modifiers = modifiers.to_vec();
        modifiers.sort_unstable();
        modifiers.push(key);
        modifiers.join("+")
    }

    #[test]
    fn shortcut_normalization_ignores_modifier_order() {
        assert_eq!(normalize_shortcut("Shift+Ctrl+X"), "ctrl+shift+x");
        assert_eq!(normalize_shortcut("Ctrl+Shift+X"), "ctrl+shift+x");
        assert_eq!(normalize_shortcut("Ctrl+="), "ctrl+=");
    }

    /// A shortcut belongs to one entry, except that tools sharing a single
    /// letter form a flyout group. A tool letter may not be used by a non-tool.
    #[test]
    fn shortcuts_do_not_collide() {
        let mut owners: std::collections::BTreeMap<String, Vec<Entry>> = Default::default();
        for e in load().unwrap() {
            if let Some(s) = &e.shortcut {
                owners
                    .entry(normalize_shortcut(s))
                    .or_default()
                    .push(e.clone());
            }
        }
        for (shortcut, entries) in owners {
            let all_tools = entries.iter().all(|e| e.kind == "tool");
            let single_letter = shortcut.chars().count() == 1;
            assert!(
                entries.len() == 1 || (all_tools && single_letter),
                "shortcut `{shortcut}` is used by {:?}",
                entries.iter().map(|e| &e.id).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let text = "[[item]]\nid='a'\nlabel='A'\n[[item]]\nid='a'\nlabel='B'\n";
        assert!(parse(text).is_err());
    }
}
