use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AddValueAction {
    pub value: i32,
}

impl Display for AddValueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value > 0 {
            write!(f, "Add value: +{}", self.value)
        } else {
            write!(f, "Add value: {}", self.value)
        }
    }
}

#[test]
fn add_value_positive() {
    let action = CalculatorActions::AddValueAction { value: 22 };
    let res = action.eval(13);
    assert_eq!(res, Ok(35));
}

#[test]
fn add_value_negative() {
    let action = CalculatorActions::AddValueAction { value: -22 };
    let res = action.eval(13);
    assert_eq!(res, Ok(-9));
}
