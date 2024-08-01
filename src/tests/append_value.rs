use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppendValueAction {
    pub value: i32,
}

impl Display for AppendValueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Append value: {}", self.value)
    }
}

#[test]
fn append_to_negative() {
    let action = CalculatorActions::AppendValueAction { value: 22 };
    let res = action.eval(-13);
    assert_eq!(res, Ok(-1322));
}

#[test]
fn append_to_positive() {
    let action = CalculatorActions::AppendValueAction { value: 22 };
    let res = action.eval(13);
    assert_eq!(res, Ok(1322));
}

#[test]
fn append_to_zero() {
    let action = CalculatorActions::AppendValueAction { value: 22 };
    let res = action.eval(0);
    assert_eq!(res, Ok(22));
}
