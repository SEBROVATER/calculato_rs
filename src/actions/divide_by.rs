use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DivideByAction {
    pub value: i32,
}

impl ActionEvaluation for DivideByAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if input as f64 % self.value as f64 != 0. {
            return Err("Result cant be even");
        }
        let output = input / self.value;
        Ok(output)
    }
}

impl Display for DivideByAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Divide by: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_by_zero() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(11);
        assert_eq!(res, Err("Result cant be even"));
    }
    #[test]
    fn divide_zero() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(0);
        assert_eq!(res, Ok(0));
    }
    #[test]
    fn divide_with_rem() {
        let action = DivideByAction { value: 4 };
        let res = action.eval(1);
        assert_eq!(res, Err("Result cant be even"));
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
