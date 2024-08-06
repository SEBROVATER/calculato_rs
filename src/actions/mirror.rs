use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MirrorAction {}

impl ActionEvaluation for MirrorAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let radix: i32 = 10;
        let mut input: i32 = input;
        let mut reversed: i32 = input;

        while input != 0 {
            reversed = if let Some(rev) = reversed.checked_mul(radix) {
                rev
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
            return Err("Mirror changed nothing");
        }
        if reversed > 999999 || reversed < -99999 {
            return Err("Intermediate result is bigger than 999999");
        };
        Ok(reversed)
    }
}

impl Display for MirrorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mirror")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_zero() {
        let action = MirrorAction {};
        let res = action.eval(0);
        assert_eq!(res, Err("Mirror changed nothing"));
    }

    #[test]
    fn mirror_negative() {
        let action = MirrorAction {};
        let res = action.eval(-13);
        assert_eq!(res, Ok(-1331));
    }
    #[test]
    fn mirror_positive() {
        let action = MirrorAction {};
        let res = action.eval(13);
        assert_eq!(res, Ok(1331));
    }
    #[test]
    fn mirror_with_tail_zero() {
        let action = MirrorAction {};
        let res = action.eval(130);
        assert_eq!(res, Ok(130031));
    }
}
