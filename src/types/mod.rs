use std::fmt::{Debug, Display};

pub mod int;
pub mod list;
pub mod string;
pub mod symbol;
pub mod vec;

pub use crate::types::{
    int::MalInt, list::MalList, string::MalString, symbol::MalSymbol, vec::MalVec,
};

pub trait MalType: Display + Debug {}
