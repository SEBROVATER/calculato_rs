use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MultiplyByAction {
    pub value: i32,
}

impl ActionEvaluation for MultiplyByAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input.checked_mul(self.value);
        if let Some(out) = output {
            if out == input {
                return Err("Mul changed nothing");
            }
            return Ok(out);
        };
        Err("Mul caused overflow")
    }
}

impl Display for MultiplyByAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "* {}", self.value)
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
    #[test]
    fn multiply_with_overflow() {
        let action = MultiplyByAction { value: i32::MAX };
        let res = action.eval(i32::MAX);
        assert_eq!(res, Err("Mul caused overflow"));
    }
}
