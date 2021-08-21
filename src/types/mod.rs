use std::{
    any::Any,
    fmt::{Debug, Display},
};

pub mod hashmap;
pub mod int;
pub mod keyword;
pub mod list;
pub mod string;
pub mod symbol;
pub mod vec;

pub use crate::types::{
    hashmap::MalHashMap, int::MalInt, keyword::MalKeyword, list::MalList, string::MalString,
    symbol::MalSymbol, vec::MalVec,
};

pub trait MalType: Display + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn MalType {
    pub fn as_type<T: 'static>(&self) -> Result<&T, ()> {
        match self.as_any().downcast_ref::<T>() {
            Some(int) => Ok(int),
            None => Err(()),
        }
    }
}
