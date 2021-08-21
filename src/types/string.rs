use std::fmt::Display;

use super::MalType;

#[derive(Debug)]
pub struct MalString {
    value: String,
}

impl From<String> for MalString {
    fn from(value: String) -> Self {
        MalString { value }
    }
}

impl Display for MalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl MalType for MalString {}
