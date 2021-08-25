use std::{
    any::Any,
    fmt::{Debug, Display},
};

use super::MalType;

pub struct MalKeyword {
    value: String,
}

impl From<String> for MalKeyword {
    fn from(value: String) -> Self {
        MalKeyword { value }
    }
}

impl Debug for MalKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}", self.value)
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

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(keyword) => self.value == keyword.value,
            Err(_) => false,
        }
    }
}
