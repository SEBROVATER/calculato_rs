use crate::gui::actions::AllActions;
use crate::solver::Solver;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CalculatorApp {
    output: i32,
    input: i32,
    steps: u8,
    all_actions: AllActions,
    #[serde(skip)]
    solver: Solver,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            output: 0,
            input: 0,
            steps: 1,
            all_actions: AllActions::default(),
            solver: Solver::default(),
        }
    }
}
impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_pixels_per_point(1.5);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CalculatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("calculato_rs");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.add(
                    egui::DragValue::new(&mut self.output)
                        .speed(0.5)
                        .range(-99999..=99999)
                        .prefix("Out: "),
                );

                ui.add(
                    egui::DragValue::new(&mut self.steps)
                        .speed(0.1)
                        .range(1..=99)
                        .prefix("Steps: "),
                );
            });

            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 30.),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.allocate_ui(egui::vec2(30., 30.), |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.input)
                                    .speed(0.5)
                                    .range(-99999..=99999),
                            );
                        })
                    })
                },
            );
            ui.separator();

            let columns_count: usize = (ui.available_width() / 40.).round() as usize;
            egui::Grid::new("buttons").num_columns(2).show(ui, |ui| {
                ui.allocate_ui(egui::vec2(40., 40.), |ui| {
                    ui.button("add  ");
                });
                ui.allocate_ui(egui::vec2(40., 40.), |ui| {
                    ui.button("add  ");
                });
                ui.allocate_ui(egui::vec2(40., 40.), |ui| {
                    ui.button("add  ");
                });
                ui.end_row();
                ui.allocate_ui(egui::vec2(40., 40.), |ui| {
                    ui.button("add  ");
                });

                ui.end_row();
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });

            // ui.separator();

            // ui.text_edit_singleline(&mut self.name)
            //     .labelled_by(name_label.id);
            ui.add(egui::Button::new("=>").shortcut_text("32"));
            ui.add(egui::Slider::new(&mut self.output, 0..=120).text("output"));
            if ui.button("Increment").clicked() {
                self.output += 1;
            }
            ui.label(format!("Hello '{}', output {}", self.steps, self.output));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}