use crate::actions::all::CalculatorActions;
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

    #[serde(skip)]
    solution: Option<Vec<CalculatorActions>>,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            output: 0,
            input: 0,
            steps: 1,
            all_actions: AllActions::default(),
            solver: Solver::default(),
            solution: Some(Vec::with_capacity(10)),
        }
    }
}
impl CalculatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // cc.egui_ctx.set_pixels_per_point(1.);
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
                egui::vec2(ui.available_width(), 40.),
                egui::Layout::right_to_left(egui::Align::TOP),
                |ui| {
                    ui.add_space(10.);
                    ui.allocate_ui(egui::vec2(20., 35.), |ui| {
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

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("buttons")
                    .striped(true)
                    .spacing(egui::vec2(5., 10.))
                    .show(ui, |ui| {
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| ui.add_space(40.));
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("Cancel").clicked() {
                                    self.solver.actions.pop();
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("Reset").clicked() {
                                    self.solver.actions.clear();

                                    if let Some(ref mut current_solution) = self.solution {
                                        current_solution.clear();
                                    } else {
                                        self.solution = Some(Vec::with_capacity(10));
                                    };
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("Run").clicked() {
                                    // if let Some(solution) = self.solver.evaluate() {
                                    //     let new_solution: Vec<Box<dyn ActionEvaluation>> = solution.iter()
                                    //         .map(| &action | {
                                    //             let a = Box::new(*action.deref().clone());
                                    //             a
                                    //         }).collect();
                                    //     // self.solution = Some(solution.iter()
                                    //     //     .map(|&item| Box::new(item.clone()))
                                    //     //
                                    //     //     .collect());
                                    //     self.solution = Some(new_solution);
                                    //
                                    // } else {
                                    //     self.solution = None;
                                    // };
                                    // TODO: display result
                                };
                            });
                        });
                        ui.end_row();
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                if let CalculatorActions::AddValueAction { mut value } =
                                    self.all_actions.add_value
                                {
                                    ui.add(
                                        egui::DragValue::new(&mut value)
                                            .speed(0.5)
                                            .range(-9999..=9999),
                                    );
                                    if ui.button("Add: ").clicked() {
                                        self.solver.add_action(self.all_actions.add_value.clone())
                                    };
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                if let CalculatorActions::MultiplyByAction { mut value } =
                                    self.all_actions.multiply_by
                                {
                                    ui.add(
                                        egui::DragValue::new(&mut value)
                                            .speed(0.5)
                                            .range(-999..=999),
                                    );
                                    if ui.button("Mult: ").clicked() {
                                        self.solver.add_action(self.all_actions.multiply_by.clone())
                                    };
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                if let CalculatorActions::DivideByAction { mut value } =
                                    self.all_actions.divide_by
                                {
                                    ui.add(
                                        egui::DragValue::new(&mut value)
                                            .speed(0.5)
                                            .range(-999..=999),
                                    );
                                    if ui.button("Div: ").clicked() {
                                        self.solver.add_action(self.all_actions.divide_by.clone())
                                    };
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("<<").clicked() {
                                    self.solver.add_action(self.all_actions.backspace.clone())
                                };
                            });
                        });
                        ui.end_row();

                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                if let CalculatorActions::AppendValueAction { mut value } =
                                    self.all_actions.append_value
                                {
                                    ui.add(
                                        egui::DragValue::new(&mut value).speed(0.5).range(0..=9999),
                                    );
                                    if ui.button("Insert").clicked() {
                                        self.solver
                                            .add_action(self.all_actions.append_value.clone())
                                    };
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                if let CalculatorActions::ReplaceValuesAction {
                                    mut repl_trg,
                                    mut repl_with,
                                } = self.all_actions.replace_values
                                {
                                    ui.add(
                                        egui::DragValue::new(&mut repl_trg)
                                            .speed(0.5)
                                            .range(-999..=999),
                                    );
                                    if ui.button("=>").clicked() {
                                        self.solver
                                            .add_action(self.all_actions.replace_values.clone())
                                    };
                                    ui.add(
                                        egui::DragValue::new(&mut repl_with)
                                            .speed(0.5)
                                            .range(-999..=999),
                                    );
                                }
                            });
                        });

                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("+/-").clicked() {
                                    self.solver.add_action(self.all_actions.sign_inv.clone())
                                };
                            });
                        });
                        ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                            ui.centered_and_justified(|ui| {
                                if ui.button("Sum").clicked() {
                                    self.solver.add_action(self.all_actions.sum_digits.clone())
                                };
                            });
                        });
                        ui.end_row();
                        ui.end_row();
                    });
            });

            ui.separator();
        });
    }
}