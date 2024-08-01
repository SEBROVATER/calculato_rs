use crate::actions::eval::ActionEvaluation;
use itertools::Itertools;

pub struct Solver {
    pub input: i32,
    pub output: i32,
    pub steps: u8,
    pub actions: Vec<Box<dyn ActionEvaluation>>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            input: 0,
            output: 0,
            steps: 1,
            actions: Vec::with_capacity(10),
        }
    }
}

impl Solver {
    pub fn add_action(&mut self, action: Box<dyn ActionEvaluation>) {
        self.actions.push(action);
    }
    pub fn remove_action(&mut self, _action: Box<dyn ActionEvaluation>) {
        todo!("Find by comparison and remove");
    }
    pub fn remove_action_idx(&mut self, idx: usize) {
        if idx >= self.actions.len() {
            return;
        }
        self.actions.remove(idx);
    }
    pub fn evaluate(&self) -> Option<Vec<&Box<dyn ActionEvaluation>>> {
        let mut it = self
            .actions
            .iter()
            .combinations_with_replacement(self.steps as usize)
            .filter_map(|actions| {
                if self.evaluate_one_combination(&actions) {
                    Some(actions)
                } else {
                    None
                }
            });
        // it.next()
        if let Some(actions) = it.next() {
            // let res = actions.iter().map(| action | action.clone()).collect();
            return Some(actions);
        };
        return None;
    }
    fn evaluate_one_combination(&self, actions: &Vec<&Box<dyn ActionEvaluation>>) -> bool {
        let mut start = self.input.clone();
        for action in actions {
            match action.eval(start) {
                Ok(output) => start = output,
                Err(_) => return false,
            }
        }
        return start == self.output;
    }
    pub fn build(input: i32, output: i32, steps: u8) -> Solver {
        return Solver {
            input,
            output,
            steps,
            actions: Vec::with_capacity(steps as usize),
        };
    }
}
