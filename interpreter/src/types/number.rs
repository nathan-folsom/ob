use crate::representation::Convert;

pub enum Number {
    I32,
    I64,
    F32,
    F64,
}

impl Convert for Number {
    fn to_binary(&self) -> Vec<u8> {
        match self {
            Self::I32 => vec![0x7F],
            Self::I64 => vec![0x7E],
            Self::F32 => vec![0x7D],
            Self::F64 => vec![0x7C],
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::I32 => "i32".into(),
            Self::I64 => "i64".into(),
            Self::F32 => "f32".into(),
            Self::F64 => "f64".into(),
        }
    }
}
