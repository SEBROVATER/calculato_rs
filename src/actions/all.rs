use crate::actions::add_value::AddValueAction;
use crate::actions::append_value::AppendValueAction;
use crate::actions::backspace::BackspaceAction;
use crate::actions::divide_by::DivideByAction;
use crate::actions::eval::ActionEvaluation;
use crate::actions::multiply_by::MultiplyByAction;
use crate::actions::replace_values::ReplaceValuesAction;
use crate::actions::sign_inv::SignInvAction;
use crate::actions::sum_digits::SumDigitsAction;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum CalculatorActions {
    AddValue(AddValueAction),
    MultiplyBy(MultiplyByAction),
    DivideBy(DivideByAction),
    Backspace(BackspaceAction),
    AppendValue(AppendValueAction),
    ReplaceValues(ReplaceValuesAction),
    SignInv(SignInvAction),
    // TODO: reverse
    SumDigits(SumDigitsAction),
    // TODO: shift
    // TODO: mirror
    // TODO: value change?
    // TODO: store
    // TODO: invert 10
    // TODO: portal
}

impl CalculatorActions {
    pub fn eval(&self, input: i32) -> Result<i32, &'static str> {
        match self {
            Self::AddValue(action) => action.eval(input),
            Self::MultiplyBy(action) => action.eval(input),
            Self::DivideBy(action) => action.eval(input),
            Self::Backspace(action) => action.eval(input),
            Self::AppendValue(action) => action.eval(input),
            Self::ReplaceValues(action) => action.eval(input),
            Self::SignInv(action) => action.eval(input),
            Self::SumDigits(action) => action.eval(input),
        }
    }
}
