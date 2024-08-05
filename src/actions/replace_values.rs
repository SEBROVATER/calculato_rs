use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReplaceValuesAction {
    pub repl_trg: u32,
    pub repl_with: u32,
}

impl ActionEvaluation for ReplaceValuesAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let repl_trg = self.repl_trg.to_string();
        let repl_with = self.repl_with.to_string();

        let output = input
            .to_string()
            .replace(&repl_trg, &repl_with)
            .parse::<i32>();
        if let Ok(out) = output {
            if out == input {
                return Err("Replace changed nothing");
            };
            return Ok(out);
        };
        Err("Replace caused unparseable string")
    }
}
impl Display for ReplaceValuesAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.repl_trg, self.repl_with)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repl_values_positive() {
        let action = ReplaceValuesAction {
            repl_trg: 13,
            repl_with: 31,
        };
        let res = action.eval(13);
        assert_eq!(res, Ok(31));
    }

    #[test]
    fn repl_values_missing() {
        let action = ReplaceValuesAction {
            repl_trg: 13,
            repl_with: 31,
        };
        let res = action.eval(146);
        assert_eq!(res, Err("Replace changed nothing"));
    }
}
