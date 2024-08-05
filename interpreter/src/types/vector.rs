use crate::representation::Convert;

pub struct Vector;

impl Convert for Vector {
    fn to_binary(&self) -> Vec<u8> {
        vec![0x7B]
    }
    fn to_string(&self) -> String {
        "v128".into()
    }
}
