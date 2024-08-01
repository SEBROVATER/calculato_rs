use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MultiplyByAction {
    pub value: i32,
}

impl Display for MultiplyByAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Multiply by: {}", self.value)
    }
}

#[test]
fn multiply_by_positive() {
    let action = CalculatorActions::MultiplyByAction { value: 4 };
    let res = action.eval(11);
    assert_eq!(res, Ok(44));
}

#[test]
fn multiply_by_negative() {
    let action = CalculatorActions::MultiplyByAction { value: -4 };
    let res = action.eval(11);
    assert_eq!(res, Ok(-44));
}
