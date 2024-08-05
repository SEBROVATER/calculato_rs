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
            if let Some(res_) = div.checked_div(10) {
                if let Some(rem_) = div.checked_rem(10) {
                    rem = rem_;
                } else {
                    return Err("Rem caused overflow");
                };
                div = res_;
            } else {
                return Err("Div caused overflow");
            };

            if let Some(accum_) = accum.checked_add(rem) {
                accum = accum_;
            } else {
                return Err("Add caused overflow");
            };
        }
        if accum == input {
            return Err("Sum changed nothing");
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
