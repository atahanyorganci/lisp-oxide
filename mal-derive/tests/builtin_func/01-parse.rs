use std::rc::Rc;

use mal_core::{
    env::Env,
    types::{func::MalFuncPtr, MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn add(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs + rhs))
}

fn main() {}
