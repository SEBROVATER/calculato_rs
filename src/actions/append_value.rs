use std::fmt;
use std::fmt::Display;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppendValueAction {
    pub value: i32,
}

impl ActionEvaluation for AppendValueAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = (String::new() + &input.to_string() + &self.value.abs().to_string())
            .parse::<i32>()
            .unwrap();
        Ok(output)
    }
}
impl Display for AppendValueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Append value: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_to_negative() {
        let action = AppendValueAction { value: 22 };
        let res = action.eval(-13);
        assert_eq!(res, Ok(-1322));
    }

    #[test]
    fn append_to_positive() {
        let action = AppendValueAction { value: 22 };
        let res = action.eval(13);
        assert_eq!(res, Ok(1322));
    }

    #[test]
    fn append_to_zero() {
        let action = AppendValueAction { value: 22 };
        let res = action.eval(0);
        assert_eq!(res, Ok(22));
    }
}
