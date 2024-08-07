#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use calculato_rs::CalculatorApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([476.0, 800.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "calculato_rs",
        native_options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
