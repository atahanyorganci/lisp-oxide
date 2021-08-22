use std::{any::Any, fmt::Display, ops::Index, rc::Rc};

use super::{MalSymbol, MalType};

#[derive(Debug)]
pub struct MalList {
    value: Vec<Rc<dyn MalType>>,
}

impl From<Vec<Rc<dyn MalType>>> for MalList {
    fn from(value: Vec<Rc<dyn MalType>>) -> Self {
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

impl Index<usize> for MalList {
    type Output = Rc<dyn MalType>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl MalList {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn values(&self) -> &[Rc<dyn MalType>] {
        self.value.as_slice()
    }

    pub fn is_def(&self) -> bool {
        if self.len() == 0 {
            return false;
        }
        match self[0].as_type::<MalSymbol>() {
            Ok(symbol) => symbol.is_def(),
            Err(_) => false,
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
}
