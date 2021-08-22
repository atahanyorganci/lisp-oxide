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

pub fn prn(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    if !args.is_empty() {
        print!("{}", args[0]);
        for arg in &args[1..] {
            print!(" {}", arg);
        }
        print!("\n");
    }
    Ok(MalNil::new())
}

pub fn list(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    Ok(Rc::from(MalList::from(Vec::from(args))))
}

pub fn is_list(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    Ok(Rc::from(MalBool::from(args[0].as_array().is_ok())))
}

pub fn is_empty(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let value = match args[0].as_array() {
        Ok(arr) => arr.is_empty(),
        Err(_) => true,
    };
    Ok(Rc::from(MalBool::from(value)))
}

pub fn count(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let value = match args[0].as_array() {
        Ok(arr) => arr.len() as i64,
        Err(_) => 0,
    };
    Ok(Rc::from(MalInt::from(value)))
}

pub fn equal(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    if args.len() != 2 {
        return Err(MalError::TypeError);
    }
    let rhs = args[0].as_ref();
    let lhs = args[1].as_ref();
    Ok(Rc::from(MalBool::from(lhs.equal(rhs))))
}

pub fn lt(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs < rhs)))
}

pub fn leq(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs <= rhs)))
}

pub fn gt(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs > rhs)))
}

pub fn geq(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs >= rhs)))
}
