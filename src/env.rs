use std::{collections::HashMap, rc::Rc};

use crate::{
    eval, eval_ast,
    types::{func::MalFuncPtr, MalClojure, MalFunc, MalInt, MalList, MalNil, MalSymbol, MalType},
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
        env.register_func("do", &do_fn);
        env.register_func("if", &if_fn);
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

    pub fn push_and_init(
        &mut self,
        symbols: &[Rc<dyn MalType>],
        values: &[Rc<dyn MalType>],
    ) -> Result<(), MalError> {
        if symbols.len() != values.len() {
            return Err(MalError::TypeError);
        }
        let mut layer = HashMap::new();
        for (symbol, value) in symbols.iter().zip(values) {
            let symbol: &MalSymbol = symbol.as_type()?;
            // FIXME: Find a way to avoid allocation
            layer.insert(symbol.clone(), value.clone());
        }
        self.env.push(layer);
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

pub fn do_fn(args: &[Rc<dyn MalType>], env: &mut Env) -> MalResult {
    let mut result: Rc<dyn MalType> = MalNil::new();
    for arg in args {
        result = eval_ast(arg.clone(), env)?;
    }
    Ok(result)
}

pub fn if_fn(args: &[Rc<dyn MalType>], env: &mut Env) -> MalResult {
    if args.len() != 2 && args.len() != 3 {
        return Err(MalError::TypeError);
    }
    if args[0].truthy() {
        eval(args[1].clone(), env)
    } else if args.len() == 3 {
        eval(args[2].clone(), env)
    } else {
        Ok(MalNil::new())
    }
}

pub fn fn_fn(args: &[Rc<dyn MalType>], _env: &mut Env) -> MalResult {
    if args.len() != 2 {
        return Err(MalError::TypeError);
    }
    let arg_names = match args[0].as_type::<MalList>() {
        Ok(list) => list.values(),
        Err(_) => todo!(),
    };
    MalClojure::try_new(arg_names, args[1].clone())
}
