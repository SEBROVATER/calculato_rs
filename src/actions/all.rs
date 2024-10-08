use crate::actions::add_value::AddValueAction;
use crate::actions::append_value::AppendValueAction;
use crate::actions::backspace::BackspaceAction;
use crate::actions::divide_by::DivideByAction;
use crate::actions::eval::ActionEvaluation;
use crate::actions::increment_buttons::IncrementButtonAction;
use crate::actions::inv10::Inv10Action;
use crate::actions::mirror::MirrorAction;
use crate::actions::multiply_by::MultiplyByAction;
use crate::actions::portal::PortalAction;
use crate::actions::pow::PowAction;
use crate::actions::replace_values::ReplaceValuesAction;
use crate::actions::reverse::ReverseAction;
use crate::actions::shift_l::ShiftLAction;
use crate::actions::shift_r::ShiftRAction;
use crate::actions::sign_inv::SignInvAction;
use crate::actions::store::{StoreValueAction, UnstoreValueAction};
use crate::actions::sum_digits::SumDigitsAction;

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorActions {
    AddValue(AddValueAction),
    MultiplyBy(MultiplyByAction),
    DivideBy(DivideByAction),
    Backspace(BackspaceAction),
    AppendValue(AppendValueAction),
    ReplaceValues(ReplaceValuesAction),
    Pow(PowAction),
    SignInv(SignInvAction),
    Reverse(ReverseAction),
    SumDigits(SumDigitsAction),
    ShiftL(ShiftLAction),
    ShiftR(ShiftRAction),
    Mirror(MirrorAction),
    IncrementButtons(IncrementButtonAction),
    StoreValue(StoreValueAction),
    UnstoreValue(UnstoreValueAction),
    Inv10(Inv10Action),
    Portal(PortalAction),
}

impl CalculatorActions {
    pub fn as_string(&self) -> String {
        match self {
            Self::AddValue(action) => format!("{}", action),
            Self::MultiplyBy(action) => format!("{}", action),
            Self::DivideBy(action) => format!("{}", action),
            Self::Backspace(action) => format!("{}", action),
            Self::AppendValue(action) => format!("{}", action),
            Self::ReplaceValues(action) => format!("{}", action),
            Self::SignInv(action) => format!("{}", action),
            Self::SumDigits(action) => format!("{}", action),
            Self::Pow(action) => format!("{}", action),
            Self::Reverse(action) => format!("{}", action),
            Self::ShiftL(action) => format!("{}", action),
            Self::ShiftR(action) => format!("{}", action),
            Self::Mirror(action) => format!("{}", action),
            Self::IncrementButtons(action) => format!("{}", action),
            Self::StoreValue(action) => format!("{}", action),
            Self::UnstoreValue(action) => format!("{}", action),
            Self::Inv10(action) => format!("{}", action),
            Self::Portal(action) => format!("{}", action),
        }
    }

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
            Self::Pow(action) => action.eval(input),
            Self::Reverse(action) => action.eval(input),
            Self::ShiftL(action) => action.eval(input),
            Self::ShiftR(action) => action.eval(input),
            Self::Mirror(action) => action.eval(input),
            Self::IncrementButtons(action) => action.eval(input),
            Self::StoreValue(action) => action.eval(input),
            Self::UnstoreValue(action) => action.eval(input),
            Self::Inv10(action) => action.eval(input),
            Self::Portal(action) => action.eval(input),
        }
    }
}
