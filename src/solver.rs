use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;

use crate::actions::all::CalculatorActions;
use crate::actions::eval::ActionEvaluation;
use crate::actions::portal::PortalAction;
use crate::actions::store::{StoreValueAction, UnstoreValueAction};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Solver {
    pub input: i32,
    pub output: i32,
    pub moves: u8,
    #[serde(skip)]
    pub actions: Vec<CalculatorActions>,
    pub portals: Option<PortalAction>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            input: 0,
            output: 0,
            moves: 1,
            actions: Vec::with_capacity(10),
            portals: None,
        }
    }
}

impl Solver {
    pub fn add_action(&mut self, action: CalculatorActions) {
        match action {
            CalculatorActions::StoreValue(_) => {
                if !self
                    .actions
                    .iter()
                    .any(|s| matches!(s, CalculatorActions::StoreValue(_)))
                {
                    self.actions.push(action);
                };
            }
            CalculatorActions::UnstoreValue(_) => {
                if !self
                    .actions
                    .iter()
                    .any(|s| matches!(s, CalculatorActions::UnstoreValue(_)))
                {
                    self.actions.push(action);
                };
            }
            CalculatorActions::Portal(action_) => {
                self.portals = Some(action_.clone());
            }
            _ => self.actions.push(action),
        };
    }
    pub fn remove_action(&mut self, action: CalculatorActions) {
        if let CalculatorActions::Portal(_) = action {
            self.portals = None;
            return;
        };

        todo!("Find by comparison and remove");
    }
    pub fn remove_action_idx(&mut self, idx: usize) {
        if idx >= self.actions.len() {
            return;
        }
        self.actions.remove(idx);
    }
    pub fn evaluate(&self) -> Option<Vec<Vec<CalculatorActions>>> {
        if self.input == self.output {
            return None;
        };
        let mut solutions: Vec<Vec<CalculatorActions>> = Vec::new();
        for n in 1..self.moves {
            let inter_solutions = (0..=n)
                .map(|_| &self.actions)
                .multi_cartesian_product()
                .filter_map(|actions| self.evaluate_one_combination(&actions));
            solutions.extend(inter_solutions);
        }

        if solutions.is_empty() {
            return None;
        }
        Some(solutions)
    }
    pub fn clone_actions(actions: &Vec<&CalculatorActions>) -> Vec<CalculatorActions> {
        let mut actions_copy: Vec<CalculatorActions> = Vec::with_capacity(actions.len());
        let mut store_value: Option<Rc<RefCell<Option<u32>>>> = None;
        for action in actions {
            if let CalculatorActions::StoreValue(_) = action {
                // let store_value: Rc<RefCell<Option<u32>>> = Rc::new((*store_action.value).clone());
                store_value = Some(Rc::new(RefCell::new(None)));
                actions_copy.push(CalculatorActions::StoreValue(StoreValueAction {
                    value: store_value.as_ref().unwrap().clone(),
                }));
            } else if let CalculatorActions::UnstoreValue(_) = action {
                if let Some(value) = &store_value {
                    // unsolvable case

                    actions_copy.push(CalculatorActions::UnstoreValue(UnstoreValueAction {
                        value: (*value).clone(),
                    }));
                } else {
                    actions_copy.push(CalculatorActions::UnstoreValue(UnstoreValueAction {
                        value: Rc::new(RefCell::new(None)),
                    }));
                };
            } else {
                actions_copy.push((*action).clone());
            };
        }
        actions_copy
    }
    pub fn evaluate_one_combination(
        &self,
        actions: &Vec<&CalculatorActions>,
    ) -> Option<Vec<CalculatorActions>> {
        let mut start = self.input;

        let mut actions_copy: Vec<CalculatorActions> = Self::clone_actions(actions);

        if actions.len() > self.moves as usize {
            return None; // currently impossible case
        };
        for i in 0..actions_copy.len() {
            if let Some(portal_action) = &self.portals {
                if let Ok(value) = portal_action.eval(start) {
                    start = value;
                } else {
                    return None;
                };
            };
            let (left_mid, right) = actions_copy.split_at_mut(i + 1);
            if let CalculatorActions::IncrementButtons(action_) = left_mid.get(i).unwrap() {
                if action_.eval_on_actions(right).is_err() {
                    return None;
                };
            };
            match actions_copy.get(i).unwrap().eval(start) {
                Ok(output) => start = output,
                Err(_) => return None,
            };

            if let Some(portal_action) = &self.portals {
                if let Ok(value) = portal_action.eval(start) {
                    start = value;
                } else {
                    return None;
                };
            };
        }
        if start == self.output {
            Some(actions_copy)
        } else {
            None
        }
    }
    pub fn build(input: i32, output: i32, moves: u8) -> Solver {
        Solver {
            input,
            output,
            moves,
            actions: Vec::with_capacity(moves as usize),
            portals: None,
        }
    }
}
