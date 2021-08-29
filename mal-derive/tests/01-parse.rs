use std::rc::Rc;

use mal::{
    env::Env,
    types::{MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn add(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs + rhs))
}

fn main() {}
