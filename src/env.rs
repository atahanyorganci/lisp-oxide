use std::{cell::RefCell, collections::HashMap, env, rc::Rc};

use crate::{
    core::*,
    rep,
    types::{func::MalFuncPtr, MalFunc, MalList, MalString, MalSymbol, MalType},
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
        env.register_func("+", &mal_add);
        env.register_func("-", &mal_subtract);
        env.register_func("*", &mal_multiply);
        env.register_func("/", &mal_divide);
        env.register_func("=", &mal_equal);
        env.register_func(">", &mal_gt);
        env.register_func(">=", &mal_geq);
        env.register_func("<", &mal_lt);
        env.register_func("<=", &mal_leq);

        // List functions
        env.register_func("list", &mal_list);
        env.register_func("list?", &mal_is_list);
        env.register_func("empty?", &mal_is_empty);
        env.register_func("count", &mal_count);

        // String functions
        env.register_func("eval", &mal_eval);
        env.register_func("prn", &mal_prn);
        env.register_func("pr-str", &mal_pr_str);
        env.register_func("str", &mal_str);
        env.register_func("println", &mal_println);
        env.register_func("read-string", &mal_read_string);
        env.register_func("slurp", &mal_slurp);

        // Atom functions
        env.register_func("atom", &mal_atom);
        env.register_func("atom?", &mal_is_atom);
        env.register_func("deref", &mal_deref);
        env.register_func("reset!", &mal_reset);
        env.register_func("swap!", &mal_swap);

        // List functions
        env.register_func("cons", &mal_cons);
        env.register_func("concat", &mal_concat);
        env.register_func("vec", &mal_vec);

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
