use std::{collections::HashMap, fmt::Display, mem::MaybeUninit, rc::Rc};

use env::Env;
use mal_derive::builtin_func;
use reader::Reader;
use types::{MalClojure, MalHashMap, MalList, MalNil, MalSymbol, MalType, MalVec};

use crate::types::MalFunc;

pub mod core;
pub mod env;
pub mod reader;
pub mod types;

pub type MalResult = Result<Rc<dyn MalType>, MalError>;

#[derive(Debug)]
pub enum MalError {
    NotCallable,
    NotFound(Rc<dyn MalType>),
    EOF,
    Unbalanced,
    TypeError,
    Unimplemented,
    IOError,
}

impl Display for MalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MalError::NotCallable => write!(f, "not a function"),
            MalError::NotFound(symbol) => write!(f, "'{}' not found", symbol),
            MalError::EOF => write!(f, "end of input"),
            MalError::Unbalanced => write!(f, "unbalanced"),
            MalError::TypeError => write!(f, "type error"),
            MalError::Unimplemented => write!(f, "-- UNIMPLEMENTED --"),
            MalError::IOError => write!(f, "IO Error"),
        }
    }
}

pub fn rep(input: &str, env: &Rc<Env>) -> Result<String, MalError> {
    let ast = read(input)?;
    let result = eval(ast, env)?;
    Ok(print(result))
}

pub fn read(input: &str) -> MalResult {
    let mut reader = Reader::from(input).peekable();
    Reader::read_from(&mut reader)
}

pub fn eval(mut ast: Rc<dyn MalType>, mut env: &Rc<Env>) -> MalResult {
    let mut init = false;
    let mut outer: MaybeUninit<Rc<Env>> = MaybeUninit::uninit();
    let result = loop {
        if let Ok(list) = ast.as_type::<MalList>() {
            if list.is_empty() {
                break Ok(ast);
            } else if list.is_special("def!") {
                break mal_def(&list.values()[1..], env);
            } else if list.is_special("let*") {
                let (new_ast, new_env) = mal_let(&list.values()[1..], env)?;
                ast = new_ast;
                unsafe {
                    if init {
                        let to_drop = outer.assume_init();
                        drop(to_drop);
                    } else {
                        init = false;
                    }
                    outer = MaybeUninit::uninit();
                    outer.as_mut_ptr().write(new_env);
                    env = &*outer.as_ptr();
                }
            } else if list.is_special("do") {
                ast = mal_do(&list.values()[1..], env)?;
            } else if list.is_special("if") {
                ast = mal_if(&list.values()[1..], env)?;
            } else if list.is_special("fn*") {
                break mal_fn(&list.values()[1..], env);
            } else {
                let new_list = eval_ast(ast, env)?;
                let values = new_list.as_type::<MalList>()?.values();
                if let Ok(func) = values[0].as_type::<MalFunc>() {
                    break func.call(&values[1..], env);
                } else if let Ok(clojure) = values[0].as_type::<MalClojure>() {
                    let (new_ast, new_env) = clojure.call(&values[1..], env)?;
                    ast = new_ast;
                    unsafe {
                        if init {
                            let to_drop = outer.assume_init();
                            drop(to_drop);
                        } else {
                            init = false;
                        }
                        outer = MaybeUninit::uninit();
                        outer.as_mut_ptr().write(new_env);
                        env = &*outer.as_ptr();
                    }
                } else {
                    break Err(MalError::NotCallable);
                }
            }
        } else {
            break eval_ast(ast, env);
        }
    };
    if init {
        unsafe {
            let env = outer.assume_init();
            drop(env);
        }
    }
    result
}

pub fn print(input: Rc<dyn MalType>) -> String {
    format!("{:?}", input)
}

pub fn eval_ast(ast: Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    if let Ok(list) = ast.as_type::<MalList>() {
        let mut result = Vec::with_capacity(list.len());
        for item in list.values() {
            result.push(eval(item.clone(), env)?)
        }
        Ok(Rc::from(MalList::from(result)))
    } else if let Ok(vec) = ast.as_type::<MalVec>() {
        let mut result = Vec::with_capacity(vec.len());
        for item in vec.values() {
            result.push(eval(item.clone(), env)?)
        }
        Ok(Rc::from(MalVec::from(result)))
    } else if let Ok(map) = ast.as_type::<MalHashMap>() {
        let mut result = HashMap::with_capacity(map.len());
        for (key, value) in map.iter() {
            result.insert(key.to_string(), eval(value.clone(), env)?);
        }
        Ok(Rc::from(MalHashMap::from(result)))
    } else if ast.is::<MalSymbol>() {
        env.get(ast)
    } else {
        Ok(ast)
    }
}

#[builtin_func(name = "def")]
pub fn def_fn(symbol: &Rc<dyn MalType>, ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let value = eval(ast.clone(), env)?;
    env.set(symbol, value.clone())?;
    Ok(value)
}

#[builtin_func(name = "let")]
pub fn let_fn(
    bindings: &Rc<dyn MalType>,
    ast: &Rc<dyn MalType>,
    env: &Rc<Env>,
) -> Result<(Rc<dyn MalType>, Rc<Env>), MalError> {
    let env_list = bindings.as_array()?;
    if env_list.len() % 2 != 0 {
        return Err(MalError::TypeError);
    }

    let new_env = Env::with_outer(env.clone());
    let pair_count = env_list.len() / 2;
    for i in 0..pair_count {
        let symbol = env_list[2 * i].clone();
        let value = eval(env_list[2 * i + 1].clone(), &new_env)?;
        new_env.set(&symbol, value.clone())?;
    }
    Ok((ast.clone(), new_env))
}

#[builtin_func(name = "do")]
pub fn do_fn(args: &[Rc<dyn MalType>], env: &Rc<Env>) -> MalResult {
    if args.is_empty() {
        return Err(MalError::TypeError);
    }
    let len = args.len();
    for arg in &args[0..len - 1] {
        eval(arg.clone(), env)?;
    }
    Ok(args[len - 1].clone())
}

#[builtin_func(name = "if")]
pub fn if_fn(
    cond: &Rc<dyn MalType>,
    body: &Rc<dyn MalType>,
    ow: std::option::Option<&Rc<dyn MalType>>,
    env: &Rc<Env>,
) -> MalResult {
    if eval(cond.clone(), env)?.truthy() {
        Ok(body.clone())
    } else if let Some(ow) = ow {
        Ok(ow.clone())
    } else {
        Ok(MalNil::new())
    }
}

#[builtin_func(name = "fn")]
pub fn fn_fn(names: &Rc<dyn MalType>, body: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    MalClojure::try_new(names.as_array()?, body.clone(), env.clone())
}
