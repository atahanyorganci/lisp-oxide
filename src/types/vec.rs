use std::fmt::Display;

use super::{MalType, MalTypeHint};

#[derive(Debug)]
pub struct MalVec {
    value: Vec<Box<dyn MalType>>,
}

impl From<Vec<Box<dyn MalType>>> for MalVec {
    fn from(value: Vec<Box<dyn MalType>>) -> Self {
        MalVec { value }
    }
}

impl Display for MalVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some(item) => write!(f, "{}", item)?,
            None => return write!(f, "]"),
        }
        for item in iter {
            write!(f, " {}", item)?;
        }
        write!(f, "]")
    }
}

impl MalVec {
    pub fn append(&mut self, item: Box<dyn MalType>) {
        self.value.push(item);
    }
}

impl MalType for MalVec {
    fn type_hint(&self) -> MalTypeHint {
        MalTypeHint::Vector
    }
}
