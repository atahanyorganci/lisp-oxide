use std::{
    any::Any,
    fmt::{Debug, Display},
};

use super::MalType;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct MalSymbol {
    value: String,
}

impl<T> From<T> for MalSymbol
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl MalSymbol {
    pub fn starts_with(&self, start: &str) -> bool {
        self.value.starts_with(start)
    }
}

impl Debug for MalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<Self>() {
            Ok(symbol) => self.value == symbol.value,
            Err(_) => false,
        }
    }
}

impl PartialEq<&str> for &MalSymbol {
    fn eq(&self, other: &&str) -> bool {
        self.value.as_str() == *other
    }
}
