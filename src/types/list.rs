use std::fmt::Display;

use super::MalType;

#[derive(Debug)]
pub struct MalList {
    value: Vec<Box<dyn MalType>>,
}

impl From<Vec<Box<dyn MalType>>> for MalList {
    fn from(value: Vec<Box<dyn MalType>>) -> Self {
        MalList { value }
    }
}

impl Display for MalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some(item) => write!(f, "{}", item)?,
            None => return write!(f, ")"),
        }
        for item in iter {
            write!(f, " {}", item)?;
        }
        write!(f, ")")
    }
}

impl MalList {
    pub fn append(&mut self, item: Box<dyn MalType>) {
        self.value.push(item);
    }
}

impl MalType for MalList {}
