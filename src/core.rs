use std::rc::Rc;

use crate::{
    env::Env,
    types::{MalBool, MalInt, MalList, MalNil, MalType},
    MalError, MalResult,
};

pub fn add(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs + rhs))
}

pub fn subtract(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs - rhs))
}

pub fn multiply(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs * rhs))
}

pub fn divide(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs / rhs))
}
