use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod boolean;
pub mod func;
pub mod hashmap;
pub mod int;
pub mod keyword;
pub mod list;
pub mod string;
pub mod symbol;
pub mod vec;

pub use crate::types::{
    boolean::MalBool, func::MalFunc, hashmap::MalHashMap, int::MalInt, keyword::MalKeyword,
    list::MalList, string::MalString, symbol::MalSymbol, vec::MalVec,
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

    pub fn as_array(&self) -> Result<&[Rc<dyn MalType>], MalError> {
        if let Ok(list) = self.as_type::<MalList>() {
            Ok(list.values())
        } else if let Ok(vector) = self.as_type::<MalVec>() {
            Ok(vector.values())
        } else {
            Err(MalError::TypeError)
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
