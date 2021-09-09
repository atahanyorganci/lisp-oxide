use std::{
    any::Any,
    fmt::{Debug, Display},
    iter::FromIterator,
    rc::Rc,
};

use super::{array_equal, MalType};

pub struct MalVec {
    value: Vec<Rc<dyn MalType>>,
}

impl From<Vec<Rc<dyn MalType>>> for MalVec {
    fn from(value: Vec<Rc<dyn MalType>>) -> Self {
        MalVec { value }
    }
}

impl FromIterator<Rc<dyn MalType>> for MalVec {
    fn from_iter<T: IntoIterator<Item = Rc<dyn MalType>>>(iter: T) -> Self {
        let value = iter.into_iter().collect();
        Self { value }
    }
}

impl<'a> FromIterator<&'a Rc<dyn MalType>> for MalVec {
    fn from_iter<T: IntoIterator<Item = &'a Rc<dyn MalType>>>(iter: T) -> Self {
        let value = iter.into_iter().cloned().collect();
        Self { value }
    }
}

impl Debug for MalVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.value.iter();
        match iter.next() {
            Some(item) => write!(f, "{:?}", item)?,
            None => return write!(f, "]"),
        }
        for item in iter {
            write!(f, " {:?}", item)?;
        }
        write!(f, "]")
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
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn values(&self) -> &[Rc<dyn MalType>] {
        self.value.as_slice()
    }
}

impl MalType for MalVec {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        let lhs = self.values();
        let rhs = match rhs.as_array() {
            Ok(rhs) => rhs,
            Err(_) => return false,
        };
        array_equal(lhs, rhs)
    }
}

impl IntoIterator for MalVec {
    type Item = Rc<dyn MalType>;

    type IntoIter = std::vec::IntoIter<Rc<dyn MalType>>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}
