use std::{collections::HashMap, rc::Rc};

use crate::{
    eval,
    types::{func::MalFuncPtr, MalFunc, MalInt, MalSymbol, MalType},
    MalError, MalResult,
};

pub struct Env {
    env: Vec<HashMap<MalSymbol, Rc<dyn MalType>>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            env: vec![HashMap::new()],
        };
        env.register_func("+", &add);
        env.register_func("-", &subtract);
        env.register_func("*", &multiply);
        env.register_func("/", &divide);
        env.register_func("def!", &def_fn);
        env.register_func("let*", &let_fn);
        env
    }
}

impl Env {
    pub fn get(&self, obj: Rc<dyn MalType>) -> MalResult {
        let symbol = obj.as_type::<MalSymbol>()?;
        for layer in self.env.iter().rev() {
            match layer.get(symbol) {
                Some(rc) => return Ok(rc.clone()),
                None => {}
            }
        }
        Err(MalError::NotFound(obj.clone()))
    }

    pub fn set(&mut self, symbol: Rc<dyn MalType>, value: Rc<dyn MalType>) -> Result<(), MalError> {
        let symbol = symbol.as_type::<MalSymbol>()?;
        // FIXME: Find a way to avoid allocation
        self.env.last_mut().unwrap().insert(symbol.clone(), value);
        Ok(())
    }

    pub fn push(&mut self) {
        self.env.push(HashMap::new())
    }

    pub fn pop(&mut self) {
        self.env.pop().unwrap();
    }

    fn register_func(&mut self, name: &'static str, ptr: &'static MalFuncPtr) {
        let symbol = MalSymbol::from(name);
        let func = Rc::from(MalFunc::new(name, ptr));
        self.env.last_mut().unwrap().insert(symbol, func);
    }
}

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

pub fn def_fn(args: &[Rc<dyn MalType>], env: &mut Env) -> MalResult {
    let symbol = args[0].clone();
    let value = eval(args[1].clone(), env)?;
    env.set(symbol, value.clone())?;
    Ok(value)
}

pub fn let_fn(args: &[Rc<dyn MalType>], env: &mut Env) -> MalResult {
    if args.len() != 2 {
        return Err(MalError::TypeError);
    }
    let env_list = args[0].as_array()?;
    if env_list.len() % 2 != 0 {
        return Err(MalError::TypeError);
    }

    env.push();
    let pair_count = env_list.len() / 2;
    for i in 0..pair_count {
        let symbol = env_list[2 * i].clone();
        let value = eval(env_list[2 * i + 1].clone(), env)?;
        env.set(symbol, value.clone())?;
    }
    let value = eval(args[1].clone(), env)?;
    env.pop();
    Ok(value)
}
