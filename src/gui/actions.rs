use crate::actions::add_value::AddValueAction;
use crate::actions::append_value::AppendValueAction;
use crate::actions::backspace::BackspaceAction;
use crate::actions::divide_by::DivideByAction;
use crate::actions::increment_buttons::IncrementButtonAction;
use crate::actions::mirror::MirrorAction;
use crate::actions::multiply_by::MultiplyByAction;
use crate::actions::pow::PowAction;
use crate::actions::replace_values::ReplaceValuesAction;
use crate::actions::reverse::ReverseAction;
use crate::actions::shift_l::ShiftLAction;
use crate::actions::shift_r::ShiftRAction;
use crate::actions::sign_inv::SignInvAction;
use crate::actions::store::{StoreValueAction, UnstoreValueAction};
use crate::actions::sum_digits::SumDigitsAction;
use std::cell::RefCell;
use std::rc::Rc;
use crate::actions::inv10::Inv10Action;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AllActions {
    pub add_value: AddValueAction,
    pub divide_by: DivideByAction,
    pub multiply_by: MultiplyByAction,
    #[serde(skip)]
    pub backspace: BackspaceAction,
    pub append_value: AppendValueAction,
    pub replace_values: ReplaceValuesAction,
    pub pow: PowAction,
    #[serde(skip)]
    pub sign_inv: SignInvAction,
    #[serde(skip)]
    pub reverse: ReverseAction,
    #[serde(skip)]
    pub sum_digits: SumDigitsAction,
    #[serde(skip)]
    pub shift_l: ShiftLAction,
    #[serde(skip)]
    pub shift_r: ShiftRAction,
    #[serde(skip)]
    pub mirror: MirrorAction,
    pub increment_buttons: IncrementButtonAction,
    #[serde(skip)]
    pub store_value: StoreValueAction,
    #[serde(skip)]
    pub unstore_value: UnstoreValueAction,
    #[serde(skip)]
    pub inv10: Inv10Action,
    // TODO: invert 10
    // TODO: portal
}

impl Default for AllActions {
    fn default() -> Self {
        let store_value: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));
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
            increment_buttons: IncrementButtonAction { value: 1 },

            store_value: StoreValueAction {
                value: store_value.clone(),
            },
            unstore_value: UnstoreValueAction {
                value: store_value.clone(),
            },
            inv10: Inv10Action {},
        }
    }
}
