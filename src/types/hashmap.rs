use std::{
    any::{Any, TypeId},
    collections::{hash_map::Iter, HashMap},
    convert::TryFrom,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::MalError;

use super::{MalKeyword, MalString, MalType};

pub struct MalHashMap {
    value: HashMap<String, Rc<dyn MalType>>,
}

impl From<HashMap<String, Rc<dyn MalType>>> for MalHashMap {
    fn from(value: HashMap<String, Rc<dyn MalType>>) -> Self {
        MalHashMap { value }
    }
}

impl TryFrom<Vec<Rc<dyn MalType>>> for MalHashMap {
    type Error = MalError;

    fn try_from(value: Vec<Rc<dyn MalType>>) -> Result<Self, Self::Error> {
        if value.len() % 2 != 0 {
            return Err(MalError::Unbalanced);
        }
        let mut map = HashMap::new();
        let mut iter = value.into_iter();
        while let Some(item) = iter.next() {
            let id = item.as_ref().type_id();
            if id != TypeId::of::<MalString>() && id != TypeId::of::<MalKeyword>() {
                return Err(MalError::TypeError);
            }
            let key = item.to_string();
            let value = iter.next().unwrap();
            map.insert(key, value);
        }
        Ok(map.into())
    }
}

impl Debug for MalHashMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some((key, value)) if key.starts_with(':') => write!(f, "{} {:?}", key, value)?,
            Some((key, value)) => write!(f, "{:?} {:?}", key, value)?,
            None => return write!(f, "}}"),
        }
        for (key, value) in iter {
            write!(f, " {:?} {:?}", key, value)?;
        }
        write!(f, "}}")
    }
}

impl Display for MalHashMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some((key, value)) => write!(f, "{} {}", key, value)?,
            None => return write!(f, "}}"),
        }
        for (key, value) in iter {
            write!(f, " {} {}", key, value)?;
        }
        write!(f, "}}")
    }
}

impl MalHashMap {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn iter(&self) -> Iter<String, Rc<dyn MalType>> {
        self.value.iter()
    }
}

impl MalHashMap {
    pub fn insert(&mut self, key: Rc<dyn MalType>, value: Rc<dyn MalType>) {
        self.value.insert(key.to_string(), value);
    }
}

impl MalType for MalHashMap {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, _rhs: &dyn MalType) -> bool {
        todo!()
    }
}
