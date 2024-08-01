use std::fmt;
use std::fmt::Display;

use crate::actions::all::CalculatorActions;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReplaceValuesAction {
    pub repl_trg: i32,
    pub repl_with: i32,
}

impl Display for ReplaceValuesAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Replace values: {} -> {}", self.repl_trg, self.repl_with)
    }
}

#[test]
fn repl_values_negative() {
    let action = CalculatorActions::ReplaceValuesAction {
        repl_trg: -13,
        repl_with: 31,
    };
    let res = action.eval(-13);
    assert_eq!(res, Ok(31));
}

#[test]
fn repl_values_positive() {
    let action = CalculatorActions::ReplaceValuesAction {
        repl_trg: 13,
        repl_with: 31,
    };
    let res = action.eval(13);
    assert_eq!(res, Ok(31));
}

#[test]
fn repl_values_missing() {
    let action = CalculatorActions::ReplaceValuesAction {
        repl_trg: 13,
        repl_with: 31,
    };
    let res = action.eval(146);
    assert_eq!(res, Ok(146));
}
