use crate::representation::Convert;

use super::{number, reference, vector};

pub enum Value {
    Number(number::Number),
    Reference(reference::Reference),
    Vector(vector::Vector),
}

impl Convert for Value {
    fn to_binary(&self) -> Vec<u8> {
        match self {
            Self::Vector(v) => v.to_binary(),
            Self::Reference(v) => v.to_binary(),
            Self::Number(v) => v.to_binary(),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Vector(v) => v.to_string(),
            Self::Reference(v) => v.to_string(),
            Self::Number(v) => v.to_string(),
        }
    }
}
