//! The engine: every user action is a command in one registry.
//!
//! Menus, shortcuts, tests, scripts and agents all run features through
//! [`Engine::execute`] by command id. See docs/recipes/add-a-command.md.

pub mod cmd;
mod params;
pub mod preview;

pub use cmd::{command_specs, CommandSpec, EngineError};
pub use editor_doc::{Document, Rgba8};
pub use params::Params;

use serde_json::Value;

/// Maximum number of undo steps kept in memory.
pub const UNDO_LIMIT: usize = 50;

/// A document plus its undo/redo history and the command registry.
pub struct Engine {
    doc: Document,
    undo: Vec<Document>,
    redo: Vec<Document>,
    specs: Vec<CommandSpec>,
}

impl Engine {
    pub fn new(doc: Document) -> Self {
        Engine {
            doc,
            undo: Vec::new(),
            redo: Vec::new(),
            specs: command_specs(),
        }
    }

    pub fn document(&self) -> &Document {
        &self.doc
    }

    pub fn specs(&self) -> &[CommandSpec] {
        &self.specs
    }

    pub fn spec(&self, id: &str) -> Option<&CommandSpec> {
        self.specs.iter().find(|s| s.id == id)
    }

    /// True when the command exists and can run on the current document.
    pub fn is_enabled(&self, id: &str) -> bool {
        self.spec(id).is_some_and(|s| (s.enabled)(&self.doc))
    }

    /// Runs a command by id. On success it records exactly one undo step.
    /// On error the document is left exactly as it was.
    pub fn execute(&mut self, id: &str, params: &Value) -> Result<Value, EngineError> {
        let spec = self
            .spec(id)
            .ok_or_else(|| EngineError::UnknownCommand(id.to_string()))?;
        if !(spec.enabled)(&self.doc) {
            return Err(EngineError::Disabled(id.to_string()));
        }
        let run = spec.run;
        let mut next = self.doc.clone();
        let result = run(&mut next, &Params::new(params))?;
        let previous = std::mem::replace(&mut self.doc, next);
        self.undo.push(previous);
        if self.undo.len() > UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
        Ok(result)
    }

    /// Steps back one command. Returns false when there is nothing to undo.
    pub fn undo(&mut self) -> bool {
        match self.undo.pop() {
            Some(previous) => {
                let current = std::mem::replace(&mut self.doc, previous);
                self.redo.push(current);
                true
            }
            None => false,
        }
    }

    /// Re-applies one undone command. Returns false when there is nothing to redo.
    pub fn redo(&mut self) -> bool {
        match self.redo.pop() {
            Some(next) => {
                let current = std::mem::replace(&mut self.doc, next);
                self.undo.push(current);
                true
            }
            None => false,
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
}
