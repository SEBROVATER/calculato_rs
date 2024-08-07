use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq)]
pub struct StoreValueAction {
    pub value: Rc<RefCell<Option<u32>>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UnstoreValueAction {
    pub value: Rc<RefCell<Option<u32>>>,
}

impl ActionEvaluation for StoreValueAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if input < 0 {
            return Err("Store can't work with negatives");
        };
        *self.value.borrow_mut() = Some(input as u32);
        Ok(input)
    }
}

impl ActionEvaluation for UnstoreValueAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        if self.value.borrow().is_none() {
            return Err("Unstore can't work before store");
        };
        if let Ok(out) =
            (String::new() + &input.to_string() + &(*self.value.borrow()).ok_or("Unstore has been called without stored value")?.to_string())
                .parse::<i32>()
        {
            if out == input {
                return Err("Append changed nothing");
            };
            if !(-99999..=999999).contains(&out) {
                return Err("Intermediate result is bigger than 999999");
            };
            return Ok(out);
        };
        Err("Insert caused unparseable string")
    }
}

impl Display for StoreValueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Store")
    }
}

impl Display for UnstoreValueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = *self.value.borrow() {
            write!(f, "{}", &value)
        } else {
            write!(f, "Unstore")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_positive() {
        let action = StoreValueAction {
            value: Rc::new(RefCell::new(None)),
        };
        let res = action.eval(13);
        assert_eq!(res, Ok(13));
        assert_eq!(*action.value.borrow(), Some(13u32));
    }

    #[test]
    fn unstore_positive() {
        let action = StoreValueAction {
            value: Rc::new(RefCell::new(Some(22))),
        };
        let action2 = UnstoreValueAction {
            value: action.value.clone(),
        };
        let res = action2.eval(13);
        assert_eq!(res, Ok(1322));
        let _ = action.eval(14);
        let res2 = action2.eval(13);
        assert_eq!(res2, Ok(1314));
    }

    #[test]
    fn append_to_zero() {
        let action = StoreValueAction {
            value: Rc::new(RefCell::new(Some(22))),
        };
        let action2 = UnstoreValueAction {
            value: action.value.clone(),
        };

        let res = action2.eval(0);
        assert_eq!(res, Ok(22));
    }
}
