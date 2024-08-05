use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BackspaceAction {}
impl ActionEvaluation for BackspaceAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input.checked_div(10);
        if let Some(out) = output {
            if out == input {
                return Err("Backspace changed nothing");
            }
            return Ok(out);
        }
        Err("Div by 10 caused overflow")
    }
}
impl Display for BackspaceAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<<")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backspace_single_positive() {
        let action = BackspaceAction {};
        let res = action.eval(3);
        assert_eq!(res, Ok(0));
    }
    #[test]
    fn backspace_single_negative() {
        let action = BackspaceAction {};
        let res = action.eval(-3);
        assert_eq!(res, Ok(0));
    }
}
