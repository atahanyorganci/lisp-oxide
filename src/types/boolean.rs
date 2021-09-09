use std::{
    any::Any,
    fmt::{Debug, Display},
};

use super::MalType;

pub struct MalBool {
    value: bool,
}

impl<T> From<T> for MalBool
where
    T: Into<bool>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Debug for MalBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
