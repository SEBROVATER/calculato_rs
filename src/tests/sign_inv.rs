use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SignInvAction {}

impl Display for SignInvAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invert sign: +/-")
    }
}


#[test]
fn sign_inv_zero() {
    let action = CalculatorActions::SignInvAction {};
    let res = action.eval(0);
    assert_eq!(res, Ok(0));
}

#[test]
fn sign_inv_negative() {
    let action = CalculatorActions::SignInvAction {};
    let res = action.eval(-13);
    assert_eq!(res, Ok(13));
}

#[test]
fn sign_inv_positive() {
    let action = CalculatorActions::SignInvAction {};
    let res = action.eval(13);
    assert_eq!(res, Ok(-13));
}
