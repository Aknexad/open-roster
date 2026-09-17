// src/main.rs

// 1. Include generated Slint code
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // 2. Instantiate the window defined in appwindow.slint
    let main_window = MainWindow::new()?;



    // 4. Run the event loop
    main_window.run()
}
