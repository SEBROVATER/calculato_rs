use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SignInvAction {}

impl ActionEvaluation for SignInvAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input * -1;
        Ok(output)
    }
}

impl Display for SignInvAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invert sign: +/-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_inv_zero() {
        let action = SignInvAction {};
        let res = action.eval(0);
        assert_eq!(res, Ok(0));
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
