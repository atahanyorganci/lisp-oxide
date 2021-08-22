use std::{any::Any, fmt::Display};

use super::MalType;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct MalSymbol {
    value: String,
}

impl From<String> for MalSymbol {
    fn from(value: String) -> Self {
        MalSymbol { value }
    }
}
impl From<&str> for MalSymbol {
    fn from(value: &str) -> Self {
        let value = String::from(value);
        MalSymbol { value }
    }
}

impl Display for MalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalSymbol {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MalSymbol {
    pub fn is_def(&self) -> bool {
        self.value == "def!"
    }

    pub fn is_let(&self) -> bool {
        self.value == "let*"
    }
}
