use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddValueAction {
    pub value: i32,
}

impl ActionEvaluation for AddValueAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input + self.value;
        Ok(output)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_value_positive() {
        let action = AddValueAction { value: 22 };
        let res = action.eval(13);
        assert_eq!(res, Ok(35));
    }

    #[test]
    fn add_value_negative() {
        let action = AddValueAction { value: -22 };
        let res = action.eval(13);
        assert_eq!(res, Ok(-9));
    }
}
