use std::{
    any::Any,
    collections::{
        hash_map::{Iter, Keys, Values},
        HashMap,
    },
    convert::TryFrom,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::MalError;

use super::{MalKeyword, MalString, MalType};

#[derive(Default, Clone)]
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
        Self::try_from_iter(value.into_iter())
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
            if key.starts_with(':') {
                write!(f, " {} {:?}", key, value)?;
            } else {
                write!(f, " {:?} {:?}", key, value)?;
            }
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            value: HashMap::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn iter(&self) -> Iter<'_, String, Rc<dyn MalType>> {
        self.value.iter()
    }

    pub fn try_from_iter<T>(iter: T) -> Result<Self, MalError>
    where
        T: Iterator<Item = Rc<dyn MalType>>,
    {
        let mut result = if let (_, Some(upper)) = iter.size_hint() {
            Self::with_capacity(upper / 2)
        } else {
            MalHashMap::new()
        };
        result.insert_mut(iter)?;
        Ok(result)
    }

    fn insert_mut<T>(&mut self, mut iter: T) -> Result<(), MalError>
    where
        T: Iterator<Item = Rc<dyn MalType>>,
    {
        while let Some(item) = iter.next() {
            if item.is::<MalString>() && item.is::<MalKeyword>() {
                return Err(MalError::TypeError);
            }
            let key = item.to_string();
            let value = iter.next().unwrap();
            self.value.insert(key, value.clone());
        }
        Ok(())
    }

    pub fn insert<T>(&self, iter: T) -> Result<Self, MalError>
    where
        T: Iterator<Item = Rc<dyn MalType>>,
    {
        let initial_capacity = if let (_, Some(upper)) = iter.size_hint() {
            self.len() + upper
        } else {
            self.len()
        };
        let mut result = Self::with_capacity(initial_capacity);
        for (k, v) in &self.value {
            result.value.insert(k.clone(), v.clone());
        }
        result.insert_mut(iter)?;
        Ok(result)
    }

    pub fn remove<'a, T>(&self, iter: T) -> Result<Self, MalError>
    where
        T: Iterator<Item = &'a Rc<dyn MalType>>,
    {
        let mut result = self.clone();
        for item in iter {
            if let Ok(string) = item.as_type::<MalString>() {
                result.value.remove(&string.value);
            } else if let Ok(keyword) = item.as_type::<MalKeyword>() {
                result.value.remove(&keyword.value);
            }
        }
        Ok(result)
    }

    pub fn get(&self, key: &str) -> Option<&Rc<dyn MalType>> {
        self.value.get(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.value.contains_key(key)
    }

    pub fn keys(&self) -> Keys<'_, String, Rc<dyn MalType>> {
        self.value.keys()
    }

    pub fn values(&self) -> Values<'_, String, Rc<dyn MalType>> {
        self.value.values()
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
        false
    }
}
