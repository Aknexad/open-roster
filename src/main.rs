// src/main.rs

slint::include_modules!();

use std::path::PathBuf;

/// Returns the path to the config file: ~/.config/open-roster/config.json
fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("open-roster");
    path.push("config.json");
    path
}

/// Load the dark-mode preference from disk. Dark mode is the default.
fn load_dark_mode() -> bool {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(value) => value
                .get("dark_mode")
                .and_then(|value| value.as_bool())
                .unwrap_or(true),
            Err(_) => true,
        },
        Err(_) => true,
    }
}

/// Save the dark-mode preference to disk.
fn save_dark_mode(dark: bool) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let json = serde_json::json!({ "dark_mode": dark });
    let _ = std::fs::write(&path, json.to_string());
}

/// Handle navigation emitted by the Slint navigation menu.
fn handle_navigation(view: String) {
    println!("Navigating to: {view}");

    // Load data or perform other Rust-side work for the selected view here.
    // The visible view is updated directly by AppWindow's Slint callback.
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let _dark_mode = load_dark_mode();

    let _window_clone = main_window.clone_strong();
    main_window.on_theme_toggled(move || {
        let dark_mode = load_dark_mode();
        save_dark_mode(dark_mode);
    });

    main_window.on_navigate(|view| {
        handle_navigation(view.to_string());
    });

    main_window.run()
}
