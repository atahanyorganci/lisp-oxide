use std::fmt::Display;

use super::{MalType, MalTypeHint};

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
    fn type_hint(&self) -> MalTypeHint {
        MalTypeHint::Keyword
    }
}
