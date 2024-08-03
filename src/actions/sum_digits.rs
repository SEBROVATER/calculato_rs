use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SumDigitsAction;

impl ActionEvaluation for SumDigitsAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let mut div: i32 = input;
        let mut rem: i32;

        let mut accum: i32 = 0;
        while div != 0 {
            let res: i32 = div / 10;
            rem = div % 10;
            div = res;
            accum += rem;
        }
        Ok(accum)
    }
}

impl Display for SumDigitsAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUM")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_digits_positive() {
        let action = SumDigitsAction;
        let res = action.eval(123);
        assert_eq!(res, Ok(6));
    }

    #[test]
    fn sum_digits_negative() {
        let action = SumDigitsAction;
        let res = action.eval(-123);
        assert_eq!(res, Ok(-6));
    }
}
