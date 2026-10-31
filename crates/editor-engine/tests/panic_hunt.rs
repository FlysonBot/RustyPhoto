//! Feeds every command hostile parameters. Every call must return Ok or Err:
//! never panic, never hang. New commands are covered automatically because
//! the test loops over the whole registry.

// Test code may unwrap (AGENTS.md rule 1 applies to product code only).
#![allow(clippy::unwrap_used)]

use editor_engine::{command_specs, Document, Engine};
use serde_json::{json, Value};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Longest any single command may take on the tiny test documents.
const DEADLINE: Duration = Duration::from_secs(4);

fn hostile_params() -> Vec<Value> {
    vec![
        json!(null),
        json!({}),
        json!([]),
        json!("string"),
        json!(42),
        json!({ "layer": -1 }),
        json!({ "layer": 999_999 }),
        json!({ "layer": u64::MAX }),
        json!({ "layer": "zero" }),
        json!({ "layer": 1.5 }),
        json!({ "name": "" }),
        json!({ "name": "x".repeat(10_000) }),
        json!({ "name": 7 }),
        json!({ "radius": -5 }),
        json!({ "radius": 1e300 }),
        json!({ "amount": "NaN" }),
        json!({ "width": 0, "height": 0 }),
        json!({ "width": 1_000_000_000, "height": 1_000_000_000 }),
    ]
}

fn documents() -> Vec<Document> {
    let one_by_one = Document::new(1, 1, [0, 0, 0, 255]).unwrap();
    let mut no_layers = Document::new(2, 2, [255; 4]).unwrap();
    no_layers.layers.clear();
    let mut stale_active = Document::new(3, 2, [9; 4]).unwrap();
    stale_active.active = 42;
    vec![one_by_one, no_layers, stale_active]
}

#[test]
fn no_command_panics_or_hangs_on_hostile_input() {
    let mut failures = Vec::new();
    for spec in command_specs() {
        for doc in documents() {
            for params in hostile_params() {
                let id = spec.id;
                let (tx, rx) = mpsc::channel();
                let params_text = params.to_string();
                let doc = doc.clone();
                thread::spawn(move || {
                    let mut engine = Engine::new(doc);
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let _ = engine.execute(id, &params);
                    }));
                    let _ = tx.send(outcome.is_ok());
                });
                match rx.recv_timeout(DEADLINE) {
                    Ok(true) => {}
                    Ok(false) => failures.push(format!("{id} PANICKED with {params_text}")),
                    Err(_) => failures.push(format!("{id} HUNG (>4s) with {params_text}")),
                }
            }
        }
    }
    assert!(failures.is_empty(), "\n{}\n", failures.join("\n"));
}
