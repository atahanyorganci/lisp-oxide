use std::{
    any::Any,
    fmt::{Debug, Display},
};

use super::MalType;

pub struct MalString {
    pub value: String,
}

impl<T> From<T> for MalString
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Debug for MalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        for ch in self.value.chars() {
            match ch {
                '"' => write!(f, "\\\"")?,
                '\n' => write!(f, "\\n")?,
                '\\' => write!(f, "\\\\")?,
                '\t' => write!(f, "\\t")?,
                _ => write!(f, "{}", ch)?,
            }
        }
        write!(f, "\"")
    }
}

impl Display for MalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

impl MalString {
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}
