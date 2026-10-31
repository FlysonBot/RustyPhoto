//! Project checks. Run from anywhere in the repo:
//!
//!   cargo xtask ci          everything CI runs, in order (use before finishing)
//!   cargo xtask layers      dependency layers (xtask/layers.toml)
//!   cargo xtask catalog     write docs/generated/coverage.md and check the floor
//!   cargo xtask catalog --check   fail if coverage.md is stale (CI uses this)
//!   cargo xtask docpaths    every path mentioned in AGENTS.md and docs/ exists

mod catalog;
mod docpaths;
mod layers;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

type Result<T = ()> = std::result::Result<T, String>;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let check = args.iter().any(|a| a == "--check");
    let result = match args.first().map(String::as_str) {
        Some("ci") => ci(),
        Some("layers") => layers::run(&root()),
        Some("catalog") => catalog::run(&root(), check),
        Some("docpaths") => docpaths::run(&root()),
        _ => Err("usage: cargo xtask <ci | layers | catalog [--check] | docpaths>".into()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("\nFAILED: {e}");
            ExitCode::FAILURE
        }
    }
}

/// The repository root (the parent of this xtask folder).
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn cargo(args: &[&str]) -> Result {
    println!("\n==> cargo {}", args.join(" "));
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let status = Command::new(cargo)
        .args(args)
        .current_dir(root())
        .status()
        .map_err(|e| format!("could not run cargo: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo {}` failed", args.join(" ")))
    }
}

fn step(name: &str, f: impl FnOnce() -> Result) -> Result {
    println!("\n==> {name}");
    f()
}

/// The full gate. CI runs exactly this on Linux, Windows and macOS.
fn ci() -> Result {
    let root = root();
    cargo(&["fmt", "--all", "--check"])?;
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])?;
    step("layers", || layers::run(&root))?;
    step("catalog --check", || catalog::run(&root, true))?;
    step("docpaths", || docpaths::run(&root))?;
    cargo(&["test", "--workspace"])?;
    println!("\nAll checks passed.");
    Ok(())
}
