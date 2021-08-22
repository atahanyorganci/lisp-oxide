use std::{collections::HashMap, rc::Rc};

use crate::{
    eval,
    types::{func::MalFuncPtr, MalFunc, MalInt, MalSymbol, MalType},
};

pub struct Env {
    env: Vec<HashMap<Rc<MalSymbol>, Rc<dyn MalType>>>,
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
        env.register_func("def!", &def);
        env
    }
}

impl Env {
    pub fn get(&self, symbol: &MalSymbol) -> Result<Rc<dyn MalType>, &'static str> {
        for layer in self.env.iter().rev() {
            match layer.get(symbol) {
                Some(rc) => return Ok(rc.clone()),
                None => {}
            }
        }
        Err("not found")
    }

    pub fn set(&mut self, symbol: &MalSymbol, value: Rc<dyn MalType>) {
        let symbol = Rc::from(symbol.clone());
        self.env.last_mut().unwrap().insert(symbol, value);
    }

    fn register_func(&mut self, name: &'static str, ptr: &'static MalFuncPtr) {
        let symbol = Rc::from(MalSymbol::from(name));
        let func = Rc::from(MalFunc::new(name, ptr));
        self.env.last_mut().unwrap().insert(symbol, func);
    }
}

pub fn add(args: &[Rc<dyn MalType>], _env: &mut Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs + rhs))
}

pub fn subtract(args: &[Rc<dyn MalType>], _env: &mut Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs - rhs))
}

pub fn multiply(args: &[Rc<dyn MalType>], _env: &mut Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs * rhs))
}

pub fn divide(args: &[Rc<dyn MalType>], _env: &mut Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs / rhs))
}

pub fn def(args: &[Rc<dyn MalType>], env: &mut Env) -> Result<Rc<dyn MalType>, &'static str> {
    let symbol = args[0].as_type::<MalSymbol>().unwrap();
    // FIXME: Try not to clone the symbol to avoid allocations
    let value = eval(args[1].clone(), env)?;
    env.set(symbol, value.clone());
    Ok(value)
}
