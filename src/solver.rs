use crate::actions::all::CalculatorActions;
use itertools::Itertools;

pub struct Solver {
    pub input: i32,
    pub output: i32,
    pub moves: u8,
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
    pub fn evaluate(&self) -> Option<Vec<CalculatorActions>> {
        let mut it = (0..self.moves)
            .map(|_| &self.actions)
            .multi_cartesian_product()
            .filter_map(|actions| {
                if self.evaluate_one_combination(&actions) {
                    Some(actions)
                } else {
                    None
                }
            });
        // it.next()
        if let Some(actions) = it.next() {
            let res: Vec<CalculatorActions> =
                actions.iter().map(|&action| action.clone()).collect();

            return Some(res);
        };
        None
    }
    pub fn evaluate_one_combination(&self, actions: &Vec<&CalculatorActions>) -> bool {
        let mut start = self.input;
        for action in actions {
            match action.eval(start) {
                Ok(output) => start = output,
                Err(_) => return false,
            }
        }
        start == self.output
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
