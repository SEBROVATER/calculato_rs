use core::fmt::Debug;
use std::fmt::Display;

pub trait ActionEvaluation: Debug + Display {
    fn eval(&self, input: i32) -> Result<i32, &'static str>;
}
