// src/main.rs

// 1. Include generated Slint code
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 2. Instantiate the window defined in appwindow.slint
    let main_window = MainWindow::new()?;

    // 3. Handle callbacks/events
    let main_window_weak = main_window.as_weak();
    main_window.on_request_increase_value(move || {
        let window = main_window_weak.unwrap();
        window.set_counter(window.get_counter() + 1);
    });

    // 4. Run the event loop
    main_window.run()
}
