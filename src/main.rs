use calculato_rs::CalculatorApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 640.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "calculato_rs",
        native_options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
