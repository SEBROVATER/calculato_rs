// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

mod actions;

use crate::actions::multiply_by::MultiplyByAction;
use crate::solver::Solver;
use actions::add_value::AddValueAction;
use actions::append_value::AppendValueAction;
use actions::backspace::BackspaceAction;
use actions::replace_values::ReplaceValuesAction;
use actions::sign_inv::SignInvAction;
use actions::sum_digits::SumDigitsAction;

mod solver;

// fn main() {
//     let mut solver = Solver::build(1, 7, 3);
//
//     solver.add_action(Box::new(AddValueAction { value: 4 }));
//     solver.add_action(Box::new(AddValueAction { value: -2 }));
//     // solver.add_action(
//     //     Box::new(MultiplyByAction { value: 4 })
//     // );
//     if let Some(actions) = solver.evaluate() {
//         println!("Found solution:car");
//         for action in actions {
//             println!("{}", action);
//         }
//         return;
//     }
//     println!("Can't find solution");
// }
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "calculato_rs",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(1.5);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    output: i32,
    input: i32,
    steps: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            output: 0,
            input: 0,
            steps: 0,
        }
    }
}

impl eframe::App for MyApp {
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
