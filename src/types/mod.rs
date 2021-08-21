use std::fmt::{Debug, Display};

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MalTypeHint {
    HashMap,
    Int,
    List,
    String,
    Symbol,
    Vector,
    Keyword,
}

pub trait MalType: Display + Debug {
    fn type_hint(&self) -> MalTypeHint;
}
