use crate::actions::all::CalculatorActions;
use itertools::Itertools;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Solver {
    pub input: i32,
    pub output: i32,
    pub moves: u8,
    #[serde(skip)]
    pub actions: Vec<CalculatorActions>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            input: 0,
            output: 0,
            moves: 1,
            actions: Vec::with_capacity(10),
        }
    }
}

impl Solver {
    pub fn add_action(&mut self, action: CalculatorActions) {
        self.actions.push(action);
    }
    pub fn remove_action(&mut self, _action: CalculatorActions) {
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
        let it = (0..self.moves)
            .map(|_| &self.actions)
            .multi_cartesian_product()
            .filter_map(|actions| {
                if let Some(solution) = self.evaluate_one_combination(&actions) {
                    Some(solution)
                } else {
                    None
                }
            });
        let mut solutions:Vec<Vec<CalculatorActions>> = Vec::with_capacity(5);
        for solution in it {
            solutions.push(solution);
        };
        if solutions.len() == 0 {
            return None;
        }
        Some(solutions)
    }
    pub fn evaluate_one_combination(&self, actions: &Vec<&CalculatorActions>) -> Option<Vec<CalculatorActions>> {
        let mut start = self.input;
        let mut actions_copy: Vec<CalculatorActions> =
            actions.iter().map(|&action| action.clone()).collect();

        for i in 0..actions_copy.len() {
            let (left_mid, right) = actions_copy.split_at_mut(i+1);
            if let CalculatorActions::IncrementButtons(action_) = left_mid.get(i).unwrap() {
                if let Err(_) = action_.eval_on_actions(right) {
                    return None;

                };
            };
            match actions_copy.get(i).unwrap().eval(start) {
                Ok(output) => start = output,
                Err(_) => return None,
            }
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
        }
    }
}
