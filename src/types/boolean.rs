use std::{any::Any, fmt::Display};

use super::MalType;

#[derive(Debug)]
pub struct MalBool {
    value: bool,
}

impl From<bool> for MalBool {
    fn from(value: bool) -> Self {
        MalBool { value }
    }
}

impl Display for MalBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalBool {
    pub fn value(&self) -> bool {
        self.value
    }
}

impl MalType for MalBool {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(boolean) => self.value == boolean.value,
            Err(_) => false,
        }
    }
}
