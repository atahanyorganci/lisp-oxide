use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod atom;
pub mod boolean;
pub mod clojure;
pub mod func;
pub mod hashmap;
pub mod int;
pub mod keyword;
pub mod list;
pub mod string;
pub mod symbol;
pub mod vec;

pub use crate::types::{
    atom::MalAtom, boolean::MalBool, clojure::MalClojure, func::MalFunc, hashmap::MalHashMap,
    int::MalInt, keyword::MalKeyword, list::MalList, string::MalString, symbol::MalSymbol,
    vec::MalVec,
};
use crate::MalError;

pub trait MalType: Display + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn equal(&self, rhs: &dyn MalType) -> bool;
}

impl dyn MalType {
    pub fn as_type<T: 'static>(&self) -> Result<&T, MalError> {
        match self.as_any().downcast_ref::<T>() {
            Some(inner) => Ok(inner),
            None => Err(MalError::TypeError),
        }
    }
    pub fn as_type_mut<T: 'static>(&mut self) -> Result<&mut T, MalError> {
        match self.as_any_mut().downcast_mut::<T>() {
            Some(inner) => Ok(inner),
            None => Err(MalError::TypeError),
        }
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.as_any().is::<T>()
    }

    pub fn as_array(&self) -> Result<&[Rc<dyn MalType>], MalError> {
        if let Ok(list) = self.as_type::<MalList>() {
            Ok(list.values())
        } else if let Ok(vector) = self.as_type::<MalVec>() {
            Ok(vector.values())
        } else {
            Err(MalError::TypeError)
        }
    }

    pub fn is_special(&self, name: &'static str) -> bool {
        if let Ok(symbol) = self.as_type::<MalSymbol>() {
            symbol == name
        } else {
            false
        }
    }

    pub fn truthy(&self) -> bool {
        if self.is::<MalNil>() {
            return false;
        }
        match self.as_type::<MalBool>() {
            Ok(boolean) => boolean.value(),
            Err(_) => true,
        }
    }
}

pub fn array_equal(lhs: &[Rc<dyn MalType>], rhs: &[Rc<dyn MalType>]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    for (a, b) in lhs.iter().zip(rhs) {
        if !a.equal(b.as_ref()) {
            return false;
        }
    }
    true
}

pub struct MalNil {}

impl Debug for MalNil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nil")
    }
}

impl Display for MalNil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nil")
    }
}

impl MalType for MalNil {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        rhs.is::<MalNil>()
    }
}

impl MalNil {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
