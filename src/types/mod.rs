use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod func;
pub mod hashmap;
pub mod int;
pub mod keyword;
pub mod list;
pub mod string;
pub mod symbol;
pub mod vec;

pub use crate::types::{
    func::MalFunc, hashmap::MalHashMap, int::MalInt, keyword::MalKeyword, list::MalList,
    string::MalString, symbol::MalSymbol, vec::MalVec,
};
use crate::MalError;

pub trait MalType: Display + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn MalType {
    pub fn as_type<T: 'static>(&self) -> Result<&T, MalError> {
        match self.as_any().downcast_ref::<T>() {
            Some(int) => Ok(int),
            None => Err(MalError::TypeError),
        }
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.as_any().is::<T>()
    }
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
}

impl MalNil {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}
