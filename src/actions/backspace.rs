use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BackspaceAction {}
impl ActionEvaluation for BackspaceAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let output = input / 10;
        Ok(output)
    }
}
impl Display for BackspaceAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Backspace")
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
