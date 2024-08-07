use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;
use std::ops::AddAssign;

#[derive(Debug, Clone, PartialEq)]
pub struct Inv10Action {}

impl ActionEvaluation for Inv10Action {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {

        if input == 0 {
            return Err("Inv10 changed nothing");
        }
        let radix: i32 = 10;
        let sign: i32 = input.signum();
        let mut input: i32 = input.checked_abs().ok_or("Abs caused overflow")?;
        let mut inversed: i32 = 0;

        let mut tail = 1;
        while input != 0 {
            let rem = input.checked_rem(radix).ok_or("Rem caused overflow")?;
            input = input.checked_div(radix).ok_or("Div caused overflow")?;

            let diff = if rem == 0 { 0 } else { rem.abs_diff(radix) as i32};
            inversed = inversed.checked_add(diff.checked_mul(tail).ok_or("Mul caused overflow")?)
                .ok_or("Add caused overflow")?;

            tail = tail.checked_mul(radix).ok_or("Mul caused overflow")?;
        }
        inversed = inversed.checked_mul(sign).ok_or("Mul with sign caused overflow")?;

        if inversed == input {
            return Err("Inv10 changed nothing");
        }
        if !(-99999..=999999).contains(&inversed) {
            return Err("Intermediate result is bigger than 999999");
        };
        Ok(inversed)
    }
}

impl Display for Inv10Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inv10")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inv10_zero() {
        let action = Inv10Action {};
        let res = action.eval(0);
        assert_eq!(res, Err("Inv10 changed nothing"));
    }

    #[test]
    fn inv10_negative() {
        let action = Inv10Action {};
        let res = action.eval(-13);
        assert_eq!(res, Ok(-97));
    }
    #[test]
    fn inv10_positive() {
        let action = Inv10Action {};
        let res = action.eval(13);
        assert_eq!(res, Ok(97));
    }
    #[test]
    fn inv10_with_tail_zero() {
        let action = Inv10Action {};
        let res = action.eval(130);
        assert_eq!(res, Ok(970));
    }
}
