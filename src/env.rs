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

        env.register(MAL_ADD);
        env.register(MAL_SUBTRACT);
        env.register(MAL_MULTIPLY);
        env.register(MAL_DIVIDE);
        env.register(MAL_PRN);
        env.register(MAL_PRINTLN);
        env.register(MAL_LIST);
        env.register(MAL_IS_LIST);
        env.register(MAL_IS_EMPTY);
        env.register(MAL_COUNT);
        env.register(MAL_EQUAL);
        env.register(MAL_LT);
        env.register(MAL_LEQ);
        env.register(MAL_GT);
        env.register(MAL_GEQ);
        env.register(MAL_PR_STR);
        env.register(MAL_STR);
        env.register(MAL_READ_STRING);
        env.register(MAL_SLURP);
        env.register(MAL_ATOM);
        env.register(MAL_IS_ATOM);
        env.register(MAL_DEREF);
        env.register(MAL_RESET);
        env.register(MAL_SWAP);
        env.register(MAL_EVAL);
        env.register(MAL_CONS);
        env.register(MAL_CONCAT);
        env.register(MAL_VEC);
        env.register(MAL_NTH);
        env.register(MAL_FIRST);
        env.register(MAL_REST);
        env.register(MAL_THROW);
        env.register(MAL_APPLY);
        env.register(MAL_MAP);
        env.register(MAL_IS_NIL);
        env.register(MAL_IS_TRUE);
        env.register(MAL_IS_FALSE);
        env.register(MAL_IS_SYMBOL);
        env.register(MAL_SYMBOL);
        env.register(MAL_IS_KEYWORD);
        env.register(MAL_KEYWORD);
        env.register(MAL_VECTOR);
        env.register(MAL_IS_VECTOR);
        env.register(MAL_IS_SEQUENTIAL);
        env.register(MAL_HASH_MAP);
        env.register(MAL_IS_MAP);
        env.register(MAL_ASSOC);
        env.register(MAL_DISSOC);
        env.register(MAL_GET);
        env.register(MAL_CONTAINS);
        env.register(MAL_KEYS);
        env.register(MAL_VALS);

        rep("(def! not (fn* (a) (if a false true)))", &env).unwrap();
        rep(
            r#"(def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))"#,
            &env,
        )
        .unwrap();
        rep(r#"(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw "odd number of forms to cond")) (cons 'cond (rest (rest xs)))))))"#, &env).unwrap();
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
        let symbol = MalSymbol::from("*ARGV*");
        self.set(&symbol, argv);
    }

    pub fn get(&self, symbol: &MalSymbol) -> MalResult {
        match self.get_impl(symbol) {
            Some(value) => Ok(value),
            None => {
                let not_found = Rc::from(symbol.clone());
                Err(MalError::NotFound(not_found))
            }
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

    pub fn set(&self, symbol: &MalSymbol, value: Rc<dyn MalType>) {
        // FIXME: Find a way to avoid allocation
        self.env.borrow_mut().insert(symbol.clone(), value);
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

    fn register(&self, pair: (&'static str, &'static MalFuncPtr)) {
        let (name, ptr) = pair;
        self.register_func(name, ptr);
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
