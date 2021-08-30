use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    core::{
        add, atom, count, deref, divide, equal, eval_fn, geq, gt, is_atom, is_empty, is_list, leq,
        list, lt, multiply, pr_str, println_fn, prn, read_string, reset, slurp, str_fn, subtract,
        swap,
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
        env.register_func("eval", &eval_fn);
        env.register_func("prn", &prn);
        env.register_func("pr-str", &pr_str);
        env.register_func("str", &str_fn);
        env.register_func("println", &println_fn);
        env.register_func("read-string", &read_string);
        env.register_func("slurp", &slurp);

        // Atom functions
        env.register_func("atom", &atom);
        env.register_func("atom?", &is_atom);
        env.register_func("deref", &deref);
        env.register_func("reset!", &reset);
        env.register_func("swap!", &swap);

        rep("(def! not (fn* (a) (if a false true)))", &env).unwrap();
        rep(
            r#"(def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))"#,
            &env,
        )
        .unwrap();
        env.init_argv();

        env
    }

    pub fn init_argv(&self) {
        let argv: Vec<_> = env::args()
            .into_iter()
            .skip(2)
            .map(|s| Rc::from(MalString::from(s)) as Rc<dyn MalType>)
            .collect();

        let argv = Rc::from(MalList::from(argv));
        let symbol: Rc<dyn MalType> = Rc::from(MalSymbol::from("*ARGV*"));
        self.set(&symbol, argv).unwrap();
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

pub fn global(env: &Rc<Env>) -> &Rc<Env> {
    match &env.outer {
        Some(outer) => global(outer),
        None => env,
    }
}
