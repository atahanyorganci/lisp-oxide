use std::fmt::Display;

use super::MalType;

#[derive(Debug)]
pub struct MalSymbol {
    value: String,
}

impl From<String> for MalSymbol {
    fn from(value: String) -> Self {
        MalSymbol { value }
    }
}

impl Display for MalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalSymbol {}
