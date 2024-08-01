use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SumDigitsAction;

impl Display for SumDigitsAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sum digits")
    }
}


#[test]
fn sum_digits_positive() {
    let action = CalculatorActions::SumDigitsAction {};
    let res = action.eval(123);
    assert_eq!(res, Ok(6));
}

#[test]
fn sum_digits_negative() {
    let action = CalculatorActions::SumDigitsAction {};
    let res = action.eval(-123);
    assert_eq!(res, Ok(-6));
}
