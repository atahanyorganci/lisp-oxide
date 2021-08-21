use std::{any::Any, fmt::Display};

use super::MalType;

#[derive(Debug)]
pub struct MalKeyword {
    value: String,
}

impl From<String> for MalKeyword {
    fn from(value: String) -> Self {
        MalKeyword { value }
    }
}

impl Display for MalKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.value)
    }
}

impl MalType for MalKeyword {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
