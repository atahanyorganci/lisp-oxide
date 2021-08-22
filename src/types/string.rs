use std::{any::Any, fmt::Display};

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

impl MalType for MalString {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(string) => self.value == string.value,
            Err(_) => false,
        }
    }
}
