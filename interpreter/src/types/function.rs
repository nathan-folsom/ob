use crate::representation::Convert;

use super::result;

pub struct Function(result::Result, result::Result);

impl Convert for Function {
    fn to_binary(&self) -> Vec<u8> {
        let mut output = vec![0x60];
        output.append(&mut self.0.to_binary());
        output.append(&mut self.1.to_binary());
        output
    }
    fn to_string(&self) -> String {
        format!("( func {} {} )", self.0.to_string(), self.1.to_string())
    }
}
