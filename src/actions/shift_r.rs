use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ShiftRAction {}

impl ActionEvaluation for ShiftRAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if input == 0 {
            return Err("Shift changed nothing");
        };
        let abs = input.checked_abs().ok_or("Abs caused overflow")?;
        let sign = input.signum();

        let mut chars: Vec<char> = abs.to_string().chars().collect();
        chars.rotate_right(1);

        let out: i32 = String::from_iter(chars).parse().map_err(|_| "Shift caused unparseable string")?;
        let with_sign = out.checked_mul(sign).ok_or("Mul caused overflow")?;

        if with_sign == input {
            return Err("Shift changed nothing");
        }

        Ok(with_sign)
    }
}

impl Display for ShiftRAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shift >")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_r_zero() {
        let action = ShiftRAction {};
        let res = action.eval(0);
        assert_eq!(res, Err("Shift changed nothing"));
    }

    #[test]
    fn shift_r_one() {
        let action = ShiftRAction {};
        let res = action.eval(1);
        assert_eq!(res, Err("Shift changed nothing"));
    }

    #[test]
    fn shift_r_negative() {
        let action = ShiftRAction {};
        let res = action.eval(-134);
        assert_eq!(res, Ok(-413));
    }
    #[test]
    fn shift_r_positive() {
        let action = ShiftRAction {};
        let res = action.eval(134);
        assert_eq!(res, Ok(413));
    }
}
