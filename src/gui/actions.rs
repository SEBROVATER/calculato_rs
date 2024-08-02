use crate::actions::add_value::AddValueAction;
use crate::actions::append_value::AppendValueAction;
use crate::actions::backspace::BackspaceAction;
use crate::actions::divide_by::DivideByAction;
use crate::actions::multiply_by::MultiplyByAction;
use crate::actions::replace_values::ReplaceValuesAction;
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
    pub sign_inv: SignInvAction,
    // TODO: reverse
    pub sum_digits: SumDigitsAction,
    // TODO: shift
    // TODO: mirror
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
            sign_inv: SignInvAction {},
            sum_digits: SumDigitsAction {},
        }
    }
}
