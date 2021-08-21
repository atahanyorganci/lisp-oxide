use std::{collections::HashMap, convert::TryFrom, fmt::Display};

use super::{MalType, MalTypeHint};

#[derive(Debug)]
pub struct MalHashMap {
    value: HashMap<String, Box<dyn MalType>>,
}

impl From<HashMap<String, Box<dyn MalType>>> for MalHashMap {
    fn from(value: HashMap<String, Box<dyn MalType>>) -> Self {
        MalHashMap { value }
    }
}

impl TryFrom<Vec<Box<dyn MalType>>> for MalHashMap {
    type Error = &'static str;

    fn try_from(value: Vec<Box<dyn MalType>>) -> Result<Self, Self::Error> {
        if value.len() % 2 != 0 {
            return Err("unbalanced");
        }
        let mut map = HashMap::new();
        let mut iter = value.into_iter();
        while let Some(item) = iter.next() {
            let key = item.to_string();
            match item.type_hint() {
                MalTypeHint::HashMap
                | MalTypeHint::Int
                | MalTypeHint::List
                | MalTypeHint::Symbol
                | MalTypeHint::Vector => return Err("unallowed datatype in hashmap"),
                MalTypeHint::String => {
                    let value = iter.next().unwrap();
                    map.insert(key, value);
                }
            }
        }
        Ok(map.into())
    }
}

impl Display for MalHashMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some((key, value)) => write!(f, "{} {}", key, value)?,
            None => return write!(f, "]"),
        }
        for (key, value) in iter {
            write!(f, " {} {}", key, value)?;
        }
        write!(f, "}}")
    }
}

impl MalHashMap {
    pub fn insert(&mut self, key: Box<dyn MalType>, value: Box<dyn MalType>) {
        self.value.insert(key.to_string(), value);
    }
}

impl MalType for MalHashMap {
    fn type_hint(&self) -> MalTypeHint {
        MalTypeHint::HashMap
    }
}
