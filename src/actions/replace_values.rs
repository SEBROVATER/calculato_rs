use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct ReplaceValuesAction {
    repl_trg: i32,
    repl_with: i32,
}

impl ActionEvaluation for ReplaceValuesAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        let repl_trg = self.repl_trg.to_string();
        let repl_with = self.repl_with.abs().to_string();

        let output = input
            .to_string()
            .replace(&repl_trg, &repl_with)
            .parse::<i32>()
            .unwrap();
        Ok(output)
    }
}
impl Display for ReplaceValuesAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Replace values: {} -> {}", self.repl_trg, self.repl_with)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repl_values_negative() {
        let action = ReplaceValuesAction {
            repl_trg: -13,
            repl_with: 31,
        };
        let res = action.eval(-13);
        assert_eq!(res, Ok(31));
    }

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
        assert_eq!(res, Ok(146));
    }
}