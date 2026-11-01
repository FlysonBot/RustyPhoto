//! The desktop app as a library, so the window (`main.rs`), the
//! `--snapshot` flag and the snapshot tests all drive the same code.

pub mod app;
mod canvas;
pub mod menu;
pub mod snapshot;

pub use app::EditorApp;
