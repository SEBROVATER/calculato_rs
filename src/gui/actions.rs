use crate::actions::all::CalculatorActions;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AllActions {
    pub add_value: CalculatorActions,
    pub multiply_by: CalculatorActions,
    pub divide_by: CalculatorActions,
    pub backspace: CalculatorActions,
    pub append_value: CalculatorActions,
    pub replace_values: CalculatorActions,
    pub sign_inv: CalculatorActions,
    // TODO: reverse
    pub sum_digits: CalculatorActions,
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
            add_value: CalculatorActions::AddValueAction { value: 0 },
            multiply_by: CalculatorActions::MultiplyByAction { value: 1 },
            divide_by: CalculatorActions::DivideByAction { value: 1 },
            backspace: CalculatorActions::BackspaceAction {},
            append_value: CalculatorActions::AppendValueAction { value: 0 },
            replace_values: CalculatorActions::ReplaceValuesAction {
                repl_trg: 0,
                repl_with: 0,
            },
            sign_inv: CalculatorActions::SignInvAction {},
            sum_digits: CalculatorActions::SumDigitsAction {},
        }
    }
}
