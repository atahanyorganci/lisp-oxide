use std::{
    any::Any,
    fmt::{Debug, Display},
    iter::FromIterator,
    ops::{Deref, Index},
    rc::Rc,
};

use super::{array_equal, MalSymbol, MalType};

pub struct MalList {
    value: Vec<Rc<dyn MalType>>,
}

impl From<Vec<Rc<dyn MalType>>> for MalList {
    fn from(value: Vec<Rc<dyn MalType>>) -> Self {
        MalList { value }
    }
}

impl FromIterator<Rc<dyn MalType>> for MalList {
    fn from_iter<T: IntoIterator<Item = Rc<dyn MalType>>>(iter: T) -> Self {
        let value = iter.into_iter().collect();
        Self { value }
    }
}

impl<'a> FromIterator<&'a Rc<dyn MalType>> for MalList {
    fn from_iter<T: IntoIterator<Item = &'a Rc<dyn MalType>>>(iter: T) -> Self {
        let value = iter.into_iter().cloned().collect();
        Self { value }
    }
}

impl Debug for MalList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some(item) => write!(f, "{:?}", item)?,
            None => return write!(f, ")"),
        }
        for item in iter {
            write!(f, " {:?}", item)?;
        }
        write!(f, ")")
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

impl Index<usize> for MalList {
    type Output = Rc<dyn MalType>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl Default for MalList {
    fn default() -> Self {
        Self { value: Vec::new() }
    }
}

impl MalList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn values(&self) -> &[Rc<dyn MalType>] {
        self.value.as_slice()
    }

    pub fn get(&self, idx: usize) -> Option<&Rc<dyn MalType>> {
        self.value.get(idx)
    }

    pub fn is_special(&self, name: &'static str) -> bool {
        if self.is_empty() {
            return false;
        }
        if let Ok(symbol) = self[0].as_type::<MalSymbol>() {
            symbol == name
        } else {
            false
        }
    }
}

impl MalType for MalList {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        let rhs = match rhs.as_array() {
            Ok(rhs) => rhs,
            Err(_) => return false,
        };
        array_equal(self, rhs)
    }
}

impl Deref for MalList {
    type Target = [Rc<dyn MalType>];

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
