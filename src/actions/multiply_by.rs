use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug)]
pub struct MultiplyByAction {
    pub value: i32,
}

impl ActionEvaluation for MultiplyByAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input * self.value;
        Ok(output)
    }
}

impl Display for MultiplyByAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Multiply by: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_by_positive() {
        let action = MultiplyByAction { value: 4 };
        let res = action.eval(11);
        assert_eq!(res, Ok(44));
    }

    #[test]
    fn multiply_by_negative() {
        let action = MultiplyByAction { value: -4 };
        let res = action.eval(11);
        assert_eq!(res, Ok(-44));
    }
}
