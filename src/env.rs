use std::{collections::HashMap, rc::Rc};

use crate::{
    core::{
        add, count, divide, equal, geq, gt, is_empty, is_list, leq, list, lt, multiply, prn,
        subtract,
    },
    types::{func::MalFuncPtr, MalFunc, MalSymbol, MalType},
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
        env.register_func("prn", &prn);
        env.register_func("list", &list);
        env.register_func("list?", &is_list);
        env.register_func("empty?", &is_empty);
        env.register_func("count", &count);
        env.register_func("=", &equal);
        env.register_func(">", &gt);
        env.register_func(">=", &geq);
        env.register_func("<", &lt);
        env.register_func("<=", &leq);

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
