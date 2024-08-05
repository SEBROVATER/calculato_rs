use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SignInvAction {}

impl ActionEvaluation for SignInvAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if let Some(out) = input.checked_neg() {
            if out == input {
                return Err("SignInv changed nothing");
            }
            return Ok(out);
        };
        Err("Negation caused overflow")
    }
}

impl Display for SignInvAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+/-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_inv_zero() {
        let action = SignInvAction {};
        let res = action.eval(0);
        assert_eq!(res, Err("SignInv changed nothing"));
    }

    #[test]
    fn sign_inv_negative() {
        let action = SignInvAction {};
        let res = action.eval(-13);
        assert_eq!(res, Ok(13));
    }
    #[test]
    fn sign_inv_positive() {
        let action = SignInvAction {};
        let res = action.eval(13);
        assert_eq!(res, Ok(-13));
    }
}
