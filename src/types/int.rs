use std::{any::Any, fmt::Display};

use super::MalType;

#[derive(Debug)]
pub struct MalInt {
    value: i64,
}

impl From<i64> for MalInt {
    fn from(value: i64) -> Self {
        MalInt { value }
    }
}

impl Display for MalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalInt {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
