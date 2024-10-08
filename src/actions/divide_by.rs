use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DivideByAction {
    pub value: i32,
}

impl ActionEvaluation for DivideByAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if input as f64 % self.value as f64 != 0. {
            return Err("Div creates reminder");
        }
        let output = input.checked_div(self.value);
        if let Some(out) = output {
            if out == input {
                return Err("Div changed nothing");
            }
            return Ok(out);
        }
        Err("Div caused overflow")
    }
}

impl Display for DivideByAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/ {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_zero() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(11);
        assert_eq!(res, Err("Div creates reminder"));
    }
    #[test]
    fn divide_zero() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(0);
        assert_eq!(res, Err("Div changed nothing"));
    }
    #[test]
    fn divide_with_rem() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(1);
        assert_eq!(res, Err("Div creates reminder"));
    }
    #[test]
    fn divide_by_positive() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(8);
        assert_eq!(res, Ok(2));
    }

    #[test]
    fn divide_by_negative() {
        let action = DivideByAction { value: -4 };
        let res = action.eval(8);
        assert_eq!(res, Ok(-2));
    }
}
