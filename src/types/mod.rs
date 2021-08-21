use std::fmt::{Debug, Display};

pub mod int;
pub mod list;
pub mod string;
pub mod symbol;

pub use crate::types::{int::MalInt, list::MalList, string::MalString, symbol::MalSymbol};

pub trait MalType: Display + Debug {}
