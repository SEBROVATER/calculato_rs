use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BackspaceAction {}

impl Display for BackspaceAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Backspace")
    }
}

#[test]
fn backspace_single_positive() {
    let action = CalculatorActions::BackspaceAction {};
    let res = action.eval(3);
    assert_eq!(res, Ok(0));
}

#[test]
fn backspace_single_negative() {
    let action = CalculatorActions::BackspaceAction {};
    let res = action.eval(-3);
    assert_eq!(res, Ok(0));
}
