use crate::actions::all::CalculatorActions;
use crate::gui::actions::AllActions;
use crate::solver::Solver;
use eframe::epaint::text::TextWrapMode;
use egui::scroll_area::ScrollBarVisibility;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CalculatorApp {
    all_actions: AllActions,

    solver: Solver,

    #[serde(skip)]
    solution: Option<Vec<CalculatorActions>>,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
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
        egui::SidePanel::left("actions")
            .max_width(73.)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add(egui::Label::new("Actions:").wrap_mode(TextWrapMode::Extend));

                for (i, action) in self.solver.actions.clone().iter().enumerate() {
                    if ui.button(action.as_string()).clicked() {
                        self.solver.remove_action_idx(i);
                    };
                }
            });

        egui::SidePanel::right("solution")
            .max_width(73.)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add(egui::Label::new("Solution:").wrap_mode(TextWrapMode::Extend));

                if let Some(solution) = &self.solution {
                    for action in solution {
                        let _ = ui.button(action.as_string());
                    }
                };
                // TODO: multi-solution
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("calculato_rs");
            ui.add_space(5.0);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.add(
                    egui::DragValue::new(&mut self.solver.output)
                        .speed(0.25)
                        .range(-99999..=99999)
                        .prefix("Goal: "),
                );

                ui.add(
                    egui::DragValue::new(&mut self.solver.moves)
                        .speed(0.1)
                        .range(1..=99)
                        .prefix("Moves: "),
                );
            });

            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 35.),
                egui::Layout::right_to_left(egui::Align::TOP),
                |ui| {
                    ui.add_space(10.);
                    ui.allocate_ui(egui::vec2(20., ui.available_height()), |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.solver.input)
                                    .speed(0.25)
                                    .range(-99999..=99999),
                            );
                        })
                    });
                    ui.centered_and_justified(|ui| {
                        if self.solution.is_none() {
                            ui.label("Unsolvable");
                        };
                    });
                },
            );
            ui.separator();

            egui::ScrollArea::vertical()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    egui::Grid::new("buttons_grid")
                        // .with_row_color(| row_n, item | Some(egui::Color32::DARK_RED))
                        .striped(true)
                        .spacing(egui::vec2(10., 10.))
                        .show(ui, |ui| {
                            ui.visuals_mut().dark_mode = true;
                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("Cancel").clicked() {
                                        self.solver.actions.pop();
                                    };
                                });
                            });
                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("Clear").clicked() {
                                        self.solver.actions.clear();
                                        self.solution = Some(Vec::with_capacity(10));
                                    };
                                });
                            });
                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("Run").clicked() {
                                        if let Some(solution) = self.solver.evaluate() {
                                            self.solution = Some(solution);
                                        } else {
                                            self.solution = None;
                                        };
                                    };
                                });
                            });
                            ui.end_row();
                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.add_value.value,
                                            )
                                            .speed(0.2)
                                            .range(-999..=999),
                                        );
                                    });
                                    ui.centered_and_justified(|ui| {
                                        if ui.button("ADD").clicked() {
                                            self.solver.add_action(CalculatorActions::AddValue(
                                                self.all_actions.add_value.clone(),
                                            ))
                                        };
                                    });
                                },
                            );
                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.multiply_by.value,
                                            )
                                            .speed(0.2)
                                            .range(-999..=999),
                                        );
                                    });
                                    ui.centered_and_justified(|ui| {
                                        if ui.button("MUL").clicked() {
                                            self.solver.add_action(CalculatorActions::MultiplyBy(
                                                self.all_actions.multiply_by.clone(),
                                            ))
                                        };
                                    });
                                },
                            );
                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.divide_by.value,
                                            )
                                            .speed(0.2)
                                            .range(-999..=999),
                                        );
                                    });
                                    ui.centered_and_justified(|ui| {
                                        if ui.button("DIV").clicked() {
                                            self.solver.add_action(CalculatorActions::DivideBy(
                                                self.all_actions.divide_by.clone(),
                                            ))
                                        };
                                    });
                                },
                            );
                            ui.end_row();
                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("<<").clicked() {
                                        self.solver.add_action(CalculatorActions::Backspace(
                                            self.all_actions.backspace.clone(),
                                        ))
                                    };
                                });
                            });

                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.append_value.value,
                                            )
                                            .speed(0.2)
                                            .range(0..=999),
                                        );
                                    });
                                    ui.centered_and_justified(|ui| {
                                        if ui.button("INSRT").clicked() {
                                            self.solver.add_action(CalculatorActions::AppendValue(
                                                self.all_actions.append_value.clone(),
                                            ))
                                        };
                                    });
                                },
                            );
                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    let width = ui.available_width();
                                    ui.horizontal(|ui| {
                                        ui.style_mut().spacing.interact_size.x = (width / 2.) - 1.;
                                        ui.style_mut().spacing.item_spacing.x = 2.;

                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.replace_values.repl_trg,
                                            )
                                            .speed(0.2)
                                            .range(-999..=999),
                                        );

                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.replace_values.repl_with,
                                            )
                                            .speed(0.2)
                                            .range(-999..=999),
                                        );
                                    });

                                    ui.centered_and_justified(|ui| {
                                        if ui.button("=>").clicked() {
                                            self.solver.add_action(
                                                CalculatorActions::ReplaceValues(
                                                    self.all_actions.replace_values.clone(),
                                                ),
                                            )
                                        };
                                    });
                                },
                            );
                            ui.end_row();

                            ui.allocate_ui_with_layout(
                                egui::vec2(45., 45.),
                                egui::Layout::top_down(egui::Align::Center),
                                |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.all_actions.pow.value,
                                            )
                                                .speed(0.1)
                                                .range(0..=10),
                                        );
                                    });
                                    ui.centered_and_justified(|ui| {
                                        if ui.button("x^n").clicked() {
                                            self.solver.add_action(CalculatorActions::Pow(
                                                self.all_actions.pow.clone(),
                                            ))
                                        };
                                    });
                                },
                            );
                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("+/-").clicked() {
                                        self.solver.add_action(CalculatorActions::SignInv(
                                            self.all_actions.sign_inv.clone(),
                                        ))
                                    };
                                });
                            });


                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("Rvrs").clicked() {
                                        self.solver.add_action(CalculatorActions::Reverse(
                                            self.all_actions.reverse.clone(),
                                        ))
                                    };
                                });
                            });
                            ui.end_row();

                            ui.allocate_ui(egui::vec2(45., 45.), |ui| {
                                ui.centered_and_justified(|ui| {
                                    if ui.button("SUM").clicked() {
                                        self.solver.add_action(CalculatorActions::SumDigits(
                                            self.all_actions.sum_digits.clone(),
                                        ))
                                    };
                                });
                            });

                            ui.end_row();
                        });
                });


            ui.separator();
        });
    }
}
