use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PowAction {
    pub value: u32,
}

impl ActionEvaluation for PowAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input.checked_pow(self.value);
        if let Some(out) = output {
            if out == input {
                return Err("Pow changed nothing");
            }
            return Ok(out);
        };
        Err("Pow caused overflow")
    }
}

impl Display for PowAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x^{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow_zero() {
        let action = PowAction { value: 11 };
        let res = action.eval(0);
        assert_eq!(res, Err("Pow changed nothing"));
    }

    #[test]
    fn pow_by_zero() {
        let action = PowAction { value: 0 };
        let res = action.eval(11);
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn pow_positive_by_positive() {
        let action = PowAction { value: 3 };
        let res = action.eval(2);
        assert_eq!(res, Ok(8));
    }
    #[test]
    fn negative_by_positive_odd() {
        let action = PowAction { value: 3 };
        let res = action.eval(-2);
        assert_eq!(res, Ok(-8));
    }
    #[test]
    fn negative_by_positive_even() {
        let action = PowAction { value: 4 };
        let res = action.eval(-2);
        assert_eq!(res, Ok(16));
    }
}
