//! Checks that every repository path mentioned in backticks in AGENTS.md
//! and docs/*.md still exists, so instructions never point at moved files.

use crate::Result;
use std::path::{Path, PathBuf};

/// Only backticked text starting with one of these counts as a path.
const ROOTS: &[&str] = &[
    "crates/",
    "docs/",
    "xtask/",
    "catalog/",
    "tests/",
    ".github/",
    "AGENTS.md",
    "README.md",
];

pub fn run(root: &Path) -> Result {
    let mut files = vec![root.join("AGENTS.md")];
    collect_md(&root.join("docs"), &mut files);

    let mut problems = Vec::new();
    let mut checked = 0;
    for file in &files {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let shown = file
            .strip_prefix(root)
            .unwrap_or(file)
            .display()
            .to_string();
        if shown.contains("generated") {
            continue;
        }
        for (n, line) in text.lines().enumerate() {
            for span in backticked(line) {
                let path = span.trim_end_matches(['.', ',', ':', ')']);
                let looks_like_path = ROOTS.iter().any(|r| path.starts_with(r))
                    && !path.contains(['<', '>', '*', ' ', '{', '}']);
                if !looks_like_path {
                    continue;
                }
                checked += 1;
                if !root.join(path).exists() {
                    problems.push(format!("{shown}:{}: `{path}` does not exist", n + 1));
                }
            }
        }
    }
    if problems.is_empty() {
        println!("docpaths: {checked} paths OK");
        Ok(())
    } else {
        Err(format!(
            "docs mention paths that don't exist (fix the doc or the path):\n  {}",
            problems.join("\n  ")
        ))
    }
}

fn collect_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

fn backticked(line: &str) -> Vec<&str> {
    line.split('`').skip(1).step_by(2).collect()
}
