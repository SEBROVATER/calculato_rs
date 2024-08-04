use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReverseAction {}

impl ActionEvaluation for ReverseAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let radix = 10;
        let mut input = input.clone();
        let mut reversed = 0;

        while input != 0 {
            reversed = reversed * radix + input % radix;
            input /= radix;
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
        assert_eq!(res, Ok(0));
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
