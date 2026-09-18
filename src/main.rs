// src/main.rs

// 1. Include generated Slint code
slint::include_modules!();

use std::path::PathBuf;

/// Returns the path to the config file: ~/.config/open-roster/config.json
fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("open-roster");
    path.push("config.json");
    path
}

/// Load the dark-mode preference from disk. Returns `true` (dark) if the file
/// doesn't exist or is malformed — dark mode is the default.
fn load_dark_mode() -> bool {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(val) => val
                .get("dark_mode")
                .and_then(|v| v.as_bool())
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

fn main() -> Result<(), slint::PlatformError> {
    // 2. Instantiate the window defined in appwindow.slint
    let main_window = MainWindow::new()?;

    // 3. Restore theme preference from disk (dark-mode is the default)
    let _dark_mode = load_dark_mode();

    // 4. Persist preference whenever the user toggles the theme
    let _window_clone = main_window.clone_strong();
    main_window.on_theme_toggled(move || {
        let is_dark = load_dark_mode();
        save_dark_mode(is_dark);
    });

    // 5. Run the event loop
    main_window.run()
}
