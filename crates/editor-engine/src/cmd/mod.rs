//! The command registry.
//!
//! Each feature area lives in its own module with a `specs()` function.
//! To add a command, follow docs/recipes/add-a-command.md.

mod adjust_invert;
mod layer_new;

use crate::params::Params;
use editor_doc::{DocError, Document};
use serde_json::Value;
use std::fmt;

/// Runs a command on a working copy of the document.
pub type RunFn = fn(&mut Document, &Params) -> Result<Value, EngineError>;

/// Decides whether a command can run on the current document (greys out menus).
pub type EnabledFn = fn(&Document) -> bool;

/// Everything the app, tests and agents need to know about one command.
pub struct CommandSpec {
    /// Must match an id in catalog/catalog.toml.
    pub id: &'static str,
    pub label: &'static str,
    /// Parameter documentation, read by people and agents.
    /// Form: `name: type = default (range) — meaning`, one per line.
    pub params: &'static str,
    pub enabled: EnabledFn,
    pub run: RunFn,
}

/// Every command in the engine. Add one line per new module here.
/// This list is a known hot spot: keep edits small and sorted.
pub fn command_specs() -> Vec<CommandSpec> {
    let mut v = Vec::new();
    v.extend(adjust_invert::specs());
    v.extend(layer_new::specs());
    v
}

/// An always-true [`EnabledFn`].
pub fn always(_: &Document) -> bool {
    true
}

/// Enabled when the document has at least one layer.
pub fn has_layers(doc: &Document) -> bool {
    !doc.layers.is_empty()
}

/// Errors from running commands.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineError {
    UnknownCommand(String),
    Disabled(String),
    BadParam { name: String, reason: String },
    Doc(DocError),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::UnknownCommand(id) => write!(f, "unknown command `{id}`"),
            EngineError::Disabled(id) => write!(f, "command `{id}` is not available right now"),
            EngineError::BadParam { name, reason } => write!(f, "parameter `{name}`: {reason}"),
            EngineError::Doc(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<DocError> for EngineError {
    fn from(e: DocError) -> Self {
        EngineError::Doc(e)
    }
}
