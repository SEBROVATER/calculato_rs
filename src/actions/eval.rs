pub trait ActionEvaluation {
    fn eval(&self, input: i32) -> Result<i32, &'static str>;
}
