use crate::actions::add_value::AddValueAction;
use crate::actions::append_value::AppendValueAction;
use crate::actions::backspace::BackspaceAction;
use crate::actions::divide_by::DivideByAction;
use crate::actions::mirror::MirrorAction;
use crate::actions::multiply_by::MultiplyByAction;
use crate::actions::pow::PowAction;
use crate::actions::replace_values::ReplaceValuesAction;
use crate::actions::reverse::ReverseAction;
use crate::actions::shift_l::ShiftLAction;
use crate::actions::shift_r::ShiftRAction;
use crate::actions::sign_inv::SignInvAction;
use crate::actions::sum_digits::SumDigitsAction;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AllActions {
    pub add_value: AddValueAction,
    pub divide_by: DivideByAction,
    pub multiply_by: MultiplyByAction,
    pub backspace: BackspaceAction,
    pub append_value: AppendValueAction,
    pub replace_values: ReplaceValuesAction,
    pub pow: PowAction,
    pub sign_inv: SignInvAction,
    pub reverse: ReverseAction,
    pub sum_digits: SumDigitsAction,
    pub shift_l: ShiftLAction,
    pub shift_r: ShiftRAction,
    pub mirror: MirrorAction,
    // TODO: value change?
    // TODO: store
    // TODO: invert 10
    // TODO: portal
}

impl Default for AllActions {
    fn default() -> Self {
        Self {
            add_value: AddValueAction { value: 0 },
            divide_by: DivideByAction { value: 1 },
            multiply_by: MultiplyByAction { value: 1 },
            backspace: BackspaceAction {},
            append_value: AppendValueAction { value: 0 },
            replace_values: ReplaceValuesAction {
                repl_trg: 0,
                repl_with: 0,
            },
            pow: PowAction { value: 1 },
            sign_inv: SignInvAction {},
            reverse: ReverseAction {},
            sum_digits: SumDigitsAction {},
            shift_l: ShiftLAction {},
            shift_r: ShiftRAction {},
            mirror: MirrorAction {},
        }
    }
}
