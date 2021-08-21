use std::fmt::Display;

use super::{MalType, MalTypeHint};

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
    fn type_hint(&self) -> MalTypeHint {
        MalTypeHint::Int
    }
}
