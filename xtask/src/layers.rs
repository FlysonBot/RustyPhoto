//! Enforces xtask/layers.toml: crates depend only downward, and only the
//! app layer may use GUI crates.

use crate::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

const APP_LAYER: u8 = 6;

#[derive(Deserialize)]
struct LayerFile {
    layers: BTreeMap<String, u8>,
    gui_only: GuiOnly,
}

#[derive(Deserialize)]
struct GuiOnly {
    crates: Vec<String>,
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    dependencies: Vec<Dependency>,
}

#[derive(Deserialize)]
struct Dependency {
    name: String,
    kind: Option<String>,
}

pub fn run(root: &Path) -> Result {
    let text = std::fs::read_to_string(root.join("xtask/layers.toml"))
        .map_err(|e| format!("xtask/layers.toml: {e}"))?;
    let table: LayerFile = toml::from_str(&text).map_err(|e| format!("xtask/layers.toml: {e}"))?;

    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let output = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let meta: Metadata =
        serde_json::from_slice(&output.stdout).map_err(|e| format!("cargo metadata: {e}"))?;

    let mut problems = Vec::new();
    for pkg in meta.packages.iter().filter(|p| p.name != "xtask") {
        let Some(&layer) = table.layers.get(&pkg.name) else {
            problems.push(format!(
                "crate `{}` is not in xtask/layers.toml; add it with its layer",
                pkg.name
            ));
            continue;
        };
        for dep in &pkg.dependencies {
            if dep.kind.as_deref() == Some("dev") {
                continue; // tests may use anything
            }
            if let Some(&dep_layer) = table.layers.get(&dep.name) {
                // Strictly lower only: same-layer links would tangle crates
                // that are meant to be independent (L0 crates stay standalone).
                if dep_layer >= layer {
                    problems.push(format!(
                        "`{}` (L{layer}) depends on `{}` (L{dep_layer}); dependencies must point to a lower layer",
                        pkg.name, dep.name
                    ));
                }
            }
            if layer < APP_LAYER && table.gui_only.crates.contains(&dep.name) {
                problems.push(format!(
                    "`{}` (L{layer}) uses GUI crate `{}`; only L{APP_LAYER} may",
                    pkg.name, dep.name
                ));
            }
        }
    }

    if problems.is_empty() {
        println!("layers: {} crates OK", table.layers.len());
        Ok(())
    } else {
        Err(format!("layer violations:\n  {}", problems.join("\n  ")))
    }
}
