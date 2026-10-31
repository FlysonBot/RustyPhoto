# ADR-001: GUI library is egui (eframe)

Status: Accepted by owner on (date)

## Context
The editor is a native desktop app in pure Rust for Linux, Windows and macOS.
Most of the screen is a custom canvas; the rest is menus and docked panels.

## Options
1. **egui + eframe:** plain Rust, fastest to build with, one codebase for three OSes, wgpu integration for a custom canvas, a test harness (egui_kittest). Default look is tool-like; breaking changes between releases.
2. **Slint:** polished widgets; requires a licensing choice; custom canvas less direct.
3. **iced:** clean architecture; slower development; custom widgets less documented.
4. **GPUI + GPUI Component:** fast, rich components; younger outside the Zed editor.

## Decision
egui via eframe. Its weak spot, styling, matters least for a dark docked professional tool; its strengths (speed of development, canvas integration, testing harness) matter most.

## Consequences
- Only L6 crates may use egui (`xtask/layers.toml`).
- egui's version is pinned in the root `Cargo.toml` and upgraded only through a new ADR, because releases change APIs.
