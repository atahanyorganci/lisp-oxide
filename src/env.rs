use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    core::{
        add, count, divide, equal, geq, gt, is_empty, is_list, leq, list, lt, multiply, pr_str,
        println_fn, prn, str_fn, subtract,
    },
    rep,
    types::{func::MalFuncPtr, MalFunc, MalSymbol, MalType},
    MalError, MalResult,
};

#[derive(Debug)]
pub struct Env {
    env: RefCell<HashMap<MalSymbol, Rc<dyn MalType>>>,
    outer: Option<Rc<Env>>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            env: RefCell::from(HashMap::new()),
            outer: None,
        }
    }
}

impl Env {
    pub fn new() -> Rc<Self> {
        let env = Rc::from(Self::default());
        // Numeric functions
        env.register_func("+", &add);
        env.register_func("-", &subtract);
        env.register_func("*", &multiply);
        env.register_func("/", &divide);
        env.register_func("=", &equal);
        env.register_func(">", &gt);
        env.register_func(">=", &geq);
        env.register_func("<", &lt);
        env.register_func("<=", &leq);

        // List functions
        env.register_func("list", &list);
        env.register_func("list?", &is_list);
        env.register_func("empty?", &is_empty);
        env.register_func("count", &count);

        // String functions
        env.register_func("prn", &prn);
        env.register_func("pr-str", &pr_str);
        env.register_func("str", &str_fn);
        env.register_func("println", &println_fn);

        match rep("(def! not (fn* (a) (if a false true)))", &env) {
            Ok(_) => env,
            Err(_) => unreachable!(),
        }
    }

    pub fn get(&self, obj: Rc<dyn MalType>) -> MalResult {
        let symbol = obj.as_type::<MalSymbol>()?;
        match self.get_impl(symbol) {
            Some(value) => Ok(value),
            None => Err(MalError::NotFound(obj.clone())),
        }
    }

    fn get_impl(&self, symbol: &MalSymbol) -> Option<Rc<dyn MalType>> {
        match self.env.borrow().get(symbol) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.get_impl(symbol),
                None => None,
            },
        }
    }

    pub fn set(&self, symbol: &Rc<dyn MalType>, value: Rc<dyn MalType>) -> Result<(), MalError> {
        let symbol = symbol.as_type::<MalSymbol>()?;
        // FIXME: Find a way to avoid allocation
        self.env.borrow_mut().insert(symbol.clone(), value);
        Ok(())
    }

    pub fn init(
        &mut self,
        symbols: &[Rc<dyn MalType>],
        values: &[Rc<dyn MalType>],
    ) -> Result<(), MalError> {
        if symbols.len() != values.len() {
            return Err(MalError::TypeError);
        }
        for (symbol, value) in symbols.iter().zip(values) {
            let symbol: &MalSymbol = symbol.as_type()?;
            // FIXME: Find a way to avoid allocation
            self.env.borrow_mut().insert(symbol.clone(), value.clone());
        }
        Ok(())
    }

    fn register_func(&self, name: &'static str, ptr: &'static MalFuncPtr) {
        let symbol = MalSymbol::from(name);
        let func = Rc::from(MalFunc::new(name, ptr));
        self.env.borrow_mut().insert(symbol, func);
    }

    pub fn with_outer(outer: Rc<Self>) -> Rc<Self> {
        Rc::from(Self {
            env: RefCell::from(HashMap::new()),
            outer: Some(outer),
        })
    }

    pub fn starts_with(&self, start: &str) -> Vec<String> {
        self.env
            .borrow()
            .keys()
            .filter(|symbol| symbol.starts_with(start))
            .map(|symbol| symbol.to_string())
            .collect()
    }
}
