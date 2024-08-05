use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReverseAction {}

impl ActionEvaluation for ReverseAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let radix: i32 = 10;
        let mut input: i32 = input;
        let mut reversed: i32 = 0;

        while input != 0 {
            if let Some(rev) = reversed.checked_mul(radix) {
                reversed = rev;
            } else {
                return Err("Mul caused overflow");
            };
            if let Some(rad) = input.checked_rem(radix) {
                if let Some(rev) = reversed.checked_add(rad) {
                    reversed = rev;
                } else {
                    return Err("Add caused overflow");
                };
            } else {
                return Err("Rem caused overflow");
            };
            if let Some(res) = input.checked_div(radix) {
                input = res;
            } else {
                return Err("Div caused overflow");
            }
        }
        if reversed == input {
            return Err("Reverse changed nothing");
        }
        Ok(reversed)
    }
}

impl Display for ReverseAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reverse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_zero() {
        let action = ReverseAction {};
        let res = action.eval(0);
        assert_eq!(res, Err("Reverse changed nothing"));
    }

    #[test]
    fn reverse_negative() {
        let action = ReverseAction {};
        let res = action.eval(-13);
        assert_eq!(res, Ok(-31));
    }
    #[test]
    fn reverse_positive() {
        let action = ReverseAction {};
        let res = action.eval(13);
        assert_eq!(res, Ok(31));
    }
    #[test]
    fn reverse_with_tail_zero() {
        let action = ReverseAction {};
        let res = action.eval(130);
        assert_eq!(res, Ok(31));
    }
}
