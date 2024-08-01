use eframe;
use calculato_rs::CalculatorApp;

mod actions;



mod solver;

fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 640.0])
            .with_min_inner_size([300.0, 600.0])
            ,
        ..Default::default()
    };
    eframe::run_native(
        "calculato_rs",
        native_options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
