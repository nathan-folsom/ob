use crate::representation::Convert;

use super::value::Value;

pub struct Result(Vec<(Value, ResultType)>);

pub enum ResultType {
    Param(Option<String>),
    Result,
}

impl Convert for Result {
    fn to_binary(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        output.push(self.0.len() as u8);
        self.0
            .iter()
            .for_each(|v| output.append(&mut v.0.to_binary()));
        output
    }
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(value, result_type)| {
                let identifier: String = match result_type {
                    ResultType::Result => "result".into(),
                    ResultType::Param(id) => {
                        if let Some(name) = id {
                            format!("param {}", name)
                        } else {
                            "param".into()
                        }
                    }
                };
                format!("( {} {} )", identifier, value.to_string())
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_to_text_param_unnamed() {
        let param = Result(vec![(
            Value::Number(crate::types::number::Number::I32),
            ResultType::Param(None),
        )]);
        let as_string = param.to_string();
        assert_eq!(as_string, "( param i32 )")
    }
    #[test]
    fn should_convert_to_text_param_named() {
        let param_name = "id".to_string();
        let param = Result(vec![(
            Value::Number(crate::types::number::Number::I32),
            ResultType::Param(Some(param_name)),
        )]);
        let as_string = param.to_string();
        assert_eq!(as_string, "( param id i32 )");
    }
    #[test]
    fn should_convert_to_text_result() {
        let param = Result(vec![(
            Value::Number(crate::types::number::Number::I32),
            ResultType::Result,
        )]);
        let as_string = param.to_string();
        assert_eq!(as_string, "( result i32 )");
    }
}
