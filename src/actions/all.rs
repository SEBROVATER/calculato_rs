#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum CalculatorActions {
    AddValueAction { value: i32 },
    MultiplyByAction { value: i32 },
    DivideByAction { value: i32 },
    BackspaceAction {},
    AppendValueAction { value: i32 },
    ReplaceValuesAction { repl_trg: i32, repl_with: i32 },
    SignInvAction {},
    // TODO: reverse
    SumDigitsAction {},
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
            Self::AddValueAction { value } => {
                let output = input + value;
                Ok(output)
            }
            Self::MultiplyByAction { value } => {
                let output = input * value;
                Ok(output)
            }
            Self::DivideByAction { value } => {
                if input as f64 % value.clone() as f64 != 0. {
                    return Err("Result cant be even");
                }
                let output = input / value;
                Ok(output)
            }
            Self::BackspaceAction {} => {
                let output = input / 10;
                Ok(output)
            }
            Self::AppendValueAction { value } => {
                let output = (String::new() + &input.to_string() + &value.abs().to_string())
                    .parse::<i32>()
                    .unwrap();
                Ok(output)
            }
            Self::ReplaceValuesAction {
                repl_trg,
                repl_with,
            } => {
                let repl_trg = repl_trg.to_string();
                let repl_with = repl_with.abs().to_string();

                let output = input
                    .to_string()
                    .replace(&repl_trg, &repl_with)
                    .parse::<i32>()
                    .unwrap();
                Ok(output)
            }
            Self::SignInvAction {} => {
                let output = -input;
                Ok(output)
            }
            Self::SumDigitsAction {} => {
                let mut div: i32 = input.clone();
                let mut rem: i32;

                let mut accum: i32 = 0;
                while div != 0 {
                    let res: i32 = div / 10;
                    rem = div % 10;
                    div = res;
                    accum += rem;
                }
                Ok(accum)
            }
        }
    }
}
