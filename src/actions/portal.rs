use crate::actions::eval::ActionEvaluation;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PortalAction {
    pub in_: u8,
    pub out_: u8,
}

impl ActionEvaluation for PortalAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if self.in_ <= self.out_ {
            return Err("Portals set incorrectly")
        };
        let in_ = self.in_ as usize;
        let out_ = self.out_ as usize;
        let sign = input.signum();
        let mut as_chars: Vec<char> = input.to_string().chars().rev().collect();
        let mut output: i32 = input;

        loop {
            let indexed = as_chars.get(in_);
            if let Some(value) =  indexed{
                if *value ==  '-' {

                    return Ok(output);
                };
                let to_add: i32 = (value.to_digit(10).ok_or("Can't parse digit char")? as i32).checked_mul(10i32.checked_pow(out_ as u32).ok_or("Pow caused overflow")?).ok_or("Mul caused overflow")?
                    .checked_mul(sign).ok_or("Mul caused  overflow")?;

                as_chars.remove(in_);
                output = String::from_iter(as_chars.iter().rev()).parse::<i32>().or(Err("Can't parse number"))?.checked_add(to_add).ok_or("Add caused overflow")?;
                as_chars = output.to_string().chars().rev().collect();
            } else {
                return Ok(output);
            };
        };

    }
}

impl Display for PortalAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) -> ({})", self.in_, self.out_)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_positive_1step() {
        let action = PortalAction {
            in_: 2,
            out_: 0,
        };
        let res = action.eval(123);
        assert_eq!(res, Ok(24));
    }

    #[test]
    fn portal_negative_1step() {
        let action = PortalAction {
            in_: 2,
            out_: 0,
        };
        let res = action.eval(-123);
        assert_eq!(res, Ok(-24));
    }
    #[test]
    fn portal_positive_2steps() {
        let action = PortalAction {
            in_: 2,
            out_: 0,
        };
        let res = action.eval(1234);
        assert_eq!(res, Ok(37));
    }

    #[test]
    fn portal_negative_2steps() {
        let action = PortalAction {
            in_: 2,
            out_: 0,
        };
        let res = action.eval(-1234);
        assert_eq!(res, Ok(-37));
    }
}
