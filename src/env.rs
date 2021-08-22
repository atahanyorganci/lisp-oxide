use std::{collections::HashMap, rc::Rc};

use crate::types::{func::MalFuncPtr, MalFunc, MalInt, MalSymbol, MalType};

pub struct Env {
    env: HashMap<Rc<MalSymbol>, Rc<dyn MalType>>,
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Self {
            env: HashMap::new(),
        };
        env.register_func("+", &add);
        env.register_func("-", &subtract);
        env.register_func("*", &multiply);
        env.register_func("/", &divide);
        env
    }
}

impl Env {
    pub fn lookup(&self, symbol: &MalSymbol) -> Result<Rc<dyn MalType>, ()> {
        match self.env.get(symbol) {
            Some(rc) => Ok(rc.clone()),
            None => Err(()),
        }
    }

    fn register_func(&mut self, name: &'static str, ptr: &'static MalFuncPtr) {
        let k = Rc::from(MalSymbol::from(name));
        let v: Rc<dyn MalType> = Rc::from(MalFunc::new(name, ptr));
        self.env.insert(k, v);
    }
}

pub fn add(args: &[Rc<dyn MalType>], _env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs + rhs))
}

pub fn subtract(args: &[Rc<dyn MalType>], _env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs - rhs))
}

pub fn multiply(args: &[Rc<dyn MalType>], _env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs * rhs))
}

pub fn divide(args: &[Rc<dyn MalType>], _env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    let lhs = args[0].as_type::<MalInt>().unwrap();
    let rhs = args[1].as_type::<MalInt>().unwrap();
    Ok(Rc::from(lhs / rhs))
}
