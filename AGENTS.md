# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Build / Run
- `cargo build` — compiles Slint UI via `build.rs` then Rust binary
- `cargo run` — runs the desktop app
- No test framework configured; no tests exist yet

## Architecture
- Rust + Slint 1.16.1 (edition 2024) desktop app
- `src/main.rs` is the only source file; uses `slint::include_modules!()` to auto-generate Rust bindings from `ui/appwindow.slint`
- `build.rs` compiles `ui/appwindow.slint` via `slint_build::compile()`
- UI declared in `ui/` as `.slint` files; styles in `ui/styles/`

## Important Notes
- README describes a much larger structure (models/, db/, services/, cloud/) that does **not** exist on disk — only `src/main.rs` is present
- `.vscode/settings.json` has `cSpell.words: ["slint"]` to suppress spelling warnings for the Slint framework name
- Slint's `include_modules!()` macro generates code at compile time; adding new `.slint` files requires a rebuild
