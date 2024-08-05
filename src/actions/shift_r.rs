use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ShiftRAction {}

impl ActionEvaluation for ShiftRAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {

        let abs: i32 = if let Some(abs_) = input.checked_abs() {
            abs_
        } else {
            return Err("Abs caused overflow");
        };
        let sign = if let Some(sign_) = input.checked_div(abs) {
            sign_
        } else if input == 0 {
            1
        } else {
            return Err("Div caused overflow");
        };

        let mut chars: Vec<char> = abs.to_string().chars().collect();

        chars.rotate_right(1);
        if let Ok(out) = String::from_iter(chars).parse::<i32>() {
            if let Some(with_sign) = out.checked_mul(sign) {
                if with_sign == input {
                    return Err("Shift changed nothing");
                };
                return Ok(with_sign);
            } else {
                return Err("Mul caused overflow");
            };
        } else {
            return Err("Shift caused unparseable string");
        };


    }
}

impl Display for ShiftRAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shift <")
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
