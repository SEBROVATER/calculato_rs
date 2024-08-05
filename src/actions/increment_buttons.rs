use std::fmt;
use std::fmt::Display;
use crate::actions::all::CalculatorActions;

use crate::actions::eval::ActionEvaluation;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct IncrementButtonAction {
    pub value: u8,
}

impl IncrementButtonAction {
    pub fn eval_on_actions(&self, actions: &mut [CalculatorActions]) -> Result<(), &'static str> {
        if self.value == 0 {
            return Err("Zero will change nothing");
        };
        for action_enum in actions.iter_mut() {
            match action_enum {
                CalculatorActions::AddValue(ref mut action) => {
                    let mut to_add = self.value as i32;
                    if action.value < 0 {
                        to_add = -to_add;
                    };
                    if let Some(new_value) = action.value.checked_add(to_add) {
                        action.value = new_value;
                    } else {
                        return Err("Add caused overflow");
                    };
                },
                CalculatorActions::MultiplyBy(ref mut action) => {
                    let mut to_add = self.value as i32;
                    if action.value < 0 {
                        to_add = -to_add;
                    };
                    if let Some(new_value) = action.value.checked_add(to_add) {
                        action.value = new_value;
                    } else {
                        return Err("Add caused overflow");
                    };
                },
                CalculatorActions::DivideBy(ref mut action) => {
                    let mut to_add = self.value as i32;
                    if action.value < 0 {
                        to_add = -to_add;
                    };
                    if let Some(new_value) = action.value.checked_add(to_add) {
                        action.value = new_value;
                    } else {
                        return Err("Add caused overflow");
                    }
                }
                CalculatorActions::AppendValue(ref mut action) => {
                    let to_add = self.value as i64;

                    if let Some(new_value) = (action.value as i64).checked_add(to_add) {
                        if  (new_value < u32::MIN as i64) || (new_value > u32::MAX as i64) {
                           return Err("Add caused overflow");
                        };
                        action.value = new_value as u32;
                    } else {
                        return Err("Add caused overflow");
                    };
                },
                CalculatorActions::ReplaceValues(ref mut action) => {
                    let to_add_trg = self.value as i64;
                    let to_add_with = self.value as i64;

                    if let Some(new_trg_value) = (action.repl_trg as i64).checked_add(to_add_trg) {
                        if  (new_trg_value < u32::MIN as i64) || (new_trg_value > u32::MAX as i64) {
                            return Err("Add caused overflow");
                        };
                        if let Some(new_with_value) = (action.repl_with as i64).checked_add(to_add_with) {
                            if  (new_with_value < u32::MIN as i64) || (new_with_value > u32::MAX as i64) {
                                return Err("Add caused overflow");
                            };
                            action.repl_trg = new_trg_value as u32;
                            action.repl_with = new_with_value as u32;
                        } else {
                            return Err("Add caused overflow");
                        };
                    } else {
                        return Err("Add caused overflow");
                    };
                },
                CalculatorActions::Pow(ref mut action) => {
                    let to_add = self.value as i64;

                    if let Some(new_value) = (action.value as i64).checked_add(to_add) {
                        if  (new_value < u32::MIN as i64) || (new_value > u32::MAX as i64) {
                            return Err("Add caused overflow");
                        };
                        action.value = new_value as u32;
                    } else {
                        return Err("Add caused overflow");
                    };
                },

                _ => {}
            }
        }
        return Ok(());

    }}

impl ActionEvaluation for IncrementButtonAction {
    fn eval(&self, input: i32) -> Result<i32, &'static str> {
        Ok(input)
    }
}

impl Display for IncrementButtonAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[+]{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::add_value::AddValueAction;
    use crate::actions::append_value::AppendValueAction;
    use crate::actions::divide_by::DivideByAction;
    use crate::actions::multiply_by::MultiplyByAction;
    use crate::actions::pow::PowAction;
    use crate::actions::replace_values::ReplaceValuesAction;
    use super::*;

    #[test]
    fn eval() {
        let action = IncrementButtonAction { value: 2 };
        assert_eq!(action.eval(4).unwrap(), 4);

    }

    #[test]
    fn increment_positive() {
        let action = IncrementButtonAction { value: 2 };
        let mut actions = vec![
            CalculatorActions::AddValue(AddValueAction { value: 5 }),
            CalculatorActions::MultiplyBy(MultiplyByAction { value: 5 }),
            CalculatorActions::DivideBy(DivideByAction { value: 5 }),
            CalculatorActions::AppendValue(AppendValueAction { value: 5 }),
            CalculatorActions::ReplaceValues(ReplaceValuesAction { repl_trg: 5, repl_with: 7 }),
            CalculatorActions::Pow(PowAction { value: 5 }),
        ];
        let res = action.eval_on_actions(&mut actions);
        assert_eq!(res, Ok(()));
        assert_eq!(&actions, &vec![
            CalculatorActions::AddValue(AddValueAction { value: 7 }),
            CalculatorActions::MultiplyBy(MultiplyByAction { value: 7 }),
            CalculatorActions::DivideBy(DivideByAction { value: 7 }),
            CalculatorActions::AppendValue(AppendValueAction { value: 7 }),
            CalculatorActions::ReplaceValues(ReplaceValuesAction { repl_trg: 7, repl_with: 9 }),
            CalculatorActions::Pow(PowAction { value: 7 }),
        ]);
    }

    #[test]
    fn increment_negative() {
        let action = IncrementButtonAction { value: 2 };
        let mut actions = vec![
            CalculatorActions::AddValue(AddValueAction { value: -5 }),
            CalculatorActions::MultiplyBy(MultiplyByAction { value: -5 }),
            CalculatorActions::DivideBy(DivideByAction { value: -5 }),
        ];
        let res = action.eval_on_actions(&mut actions);
        assert_eq!(&actions, &vec![
            CalculatorActions::AddValue(AddValueAction { value: -7 }),
            CalculatorActions::MultiplyBy(MultiplyByAction { value: -7 }),
            CalculatorActions::DivideBy(DivideByAction { value: -7 }),
        ]);
        assert_eq!(res, Ok(()));
    }
}
