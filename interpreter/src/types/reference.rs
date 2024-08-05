use crate::representation::Convert;

pub enum Reference {
    Function,
    External,
}

impl Convert for Reference {
    fn to_binary(&self) -> Vec<u8> {
        match self {
            Self::Function => vec![0x70],
            Self::External => vec![0x6F],
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Function => "funcref".into(),
            Self::External => "externref".into(),
        }
    }
}
