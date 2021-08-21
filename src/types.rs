use std::fmt::{Debug, Display};

pub trait MalType: Display + Debug {}

#[derive(Debug)]
pub struct MalInt {
    value: i64,
}

impl From<i64> for MalInt {
    fn from(value: i64) -> Self {
        MalInt { value }
    }
}

impl Display for MalInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalInt {}

#[derive(Debug)]
pub struct MalSymbol {
    value: String,
}

impl From<String> for MalSymbol {
    fn from(value: String) -> Self {
        MalSymbol { value }
    }
}

impl Display for MalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MalType for MalSymbol {}

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

impl MalType for MalString {}

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
