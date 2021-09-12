use std::{
    any::Any,
    fmt::{Debug, Display},
};

use crate::MalError;

use super::MalType;

#[derive(Clone)]
pub struct MalException {
    value: MalError,
}

impl From<MalError> for MalException {
    fn from(value: MalError) -> Self {
        Self { value }
    }
}

impl Debug for MalException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Display for MalException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalException {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(rhs) => self.value == rhs.value,
            Err(_) => false,
        }
    }
}
