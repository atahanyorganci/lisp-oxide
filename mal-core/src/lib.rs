#![deny(missing_debug_implementations, rust_2018_idioms)]
#![deny(clippy::all)]

use std::{
    collections::HashMap,
    mem::{self, MaybeUninit},
    rc::Rc,
};

use env::Env;
use mal_derive::builtin_func;
use reader::{Reader, ReaderResult};
use thiserror::Error;
use types::{
    MalClojure, MalException, MalFunc, MalHashMap, MalList, MalNil, MalSymbol, MalType, MalVec,
};

pub mod core;
pub mod env;
pub mod reader;
pub mod types;

pub type MalResult = Result<Rc<dyn MalType>, MalError>;

#[derive(Error, Debug, Clone)]
pub enum MalError {
    #[error("`{0}` is not callable.")]
    NotCallable(Rc<dyn MalType>),
    #[error("`{0}` not found in current scope.")]
    NotFound(Rc<dyn MalType>),
    #[error("Exception `{0}`")]
    Exception(Rc<dyn MalType>),
    #[error("Type error")]
    TypeError,
    #[error("Not implemented!")]
    Unimplemented,
    #[error("IOError")]
    IOError,
    #[error("{idx} is out of bounds, index should be between 0 and {len}")]
    OutOfBounds { idx: usize, len: usize },
}

impl PartialEq for MalError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NotCallable(l0), Self::NotCallable(r0)) => l0 == r0,
            (Self::NotFound(l0), Self::NotFound(r0)) => l0 == r0,
            (Self::Exception(l0), Self::Exception(r0)) => l0 == r0,
            (
                Self::OutOfBounds {
                    idx: l_idx,
                    len: l_len,
                },
                Self::OutOfBounds {
                    idx: r_idx,
                    len: r_len,
                },
            ) => l_idx == r_idx && l_len == r_len,
            _ => mem::discriminant(self) == mem::discriminant(other),
        }
    }
}

pub fn rep(input: &str, env: &Rc<Env>) -> Result<String, MalError> {
    let ast = match read(input) {
        Ok(ast) => ast,
        Err(_) => todo!(),
    };
    let result = eval(ast, env)?;
    Ok(print(result))
}

pub fn read(input: &str) -> ReaderResult {
    let mut reader = Reader::from(input).peekable();
    Reader::read_from(&mut reader)
}

pub fn eval(mut ast: Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let mut init = false;
    let mut outer: MaybeUninit<Rc<Env>> = MaybeUninit::uninit();
    let mut env = env;
    let result = loop {
        ast = macro_expand(ast, env)?;
        if let Ok(list) = ast.as_type::<MalList>() {
            if list.is_empty() {
                break Ok(ast);
            } else if list.is_special("def!") {
                break mal_def(&list.values()[1..], env);
            } else if list.is_special("defmacro!") {
                break mal_defmacro(&list.values()[1..], env);
            } else if list.is_special("let*") {
                let (new_ast, new_env) = mal_let(&list.values()[1..], env)?;
                ast = new_ast;
                unsafe {
                    if init {
                        let to_drop = outer.assume_init();
                        drop(to_drop);
                    } else {
                        init = true;
                    }
                    outer = MaybeUninit::uninit();
                    outer.write(new_env);
                    env = outer.assume_init_ref();
                }
            } else if list.is_special("macroexpand") {
                if let Some(ast) = list.get(1) {
                    break macro_expand(ast.clone(), env);
                } else {
                    return Err(MalError::TypeError);
                }
            } else if list.is_special("do") {
                ast = mal_do(&list.values()[1..], env)?;
            } else if list.is_special("if") {
                ast = mal_if(&list.values()[1..], env)?;
            } else if list.is_special("fn*") {
                break mal_fn(&list.values()[1..], env);
            } else if list.is_special("quote") {
                break mal_quote(&list.values()[1..], env);
            } else if list.is_special("quasiquote") {
                break mal_quasiquote(&list.values()[1..], env);
            } else if list.is_special("try*") {
                break mal_try(&list.values()[1..], env);
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
                            init = true;
                        }
                        outer = MaybeUninit::uninit();
                        outer.write(new_env);
                        env = outer.assume_init_ref();
                    }
                } else {
                    break Err(MalError::NotCallable(values[0].clone()));
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
    } else if let Ok(symbol) = ast.as_type() {
        env.get(symbol)
    } else {
        Ok(ast)
    }
}

#[builtin_func(name = "def", symbol = "def!", special)]
pub fn def_fn(symbol: &MalSymbol, ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let value = eval(ast.clone(), env)?;
    env.set(symbol, value.clone());
    Ok(value)
}

#[builtin_func(name = "let", symbol = "let*", special)]
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
        let symbol = env_list[2 * i].as_type()?;
        let value = eval(env_list[2 * i + 1].clone(), &new_env)?;
        new_env.set(symbol, value.clone());
    }
    Ok((ast.clone(), new_env))
}

#[builtin_func(name = "do", special)]
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

#[builtin_func(name = "if", special)]
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

#[builtin_func(name = "fn", symbol = "fn*", special)]
pub fn fn_fn(names: &Rc<dyn MalType>, body: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    MalClojure::try_new(names.as_array()?, body.clone(), env.clone())
}

#[builtin_func(special)]
pub fn quote(ast: &Rc<dyn MalType>) -> MalResult {
    Ok(ast.clone())
}

#[builtin_func(special)]
pub fn unquote(ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    eval(ast.clone(), env)
}

#[builtin_func(special)]
pub fn quasiquote(to_quote: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let elems = if let Ok(list) = to_quote.as_type::<MalList>() {
        if list.is_empty() {
            return Ok(to_quote.clone());
        } else if list.is_special("unquote") {
            return match list.get(1) {
                Some(ast) => unquote(ast, env),
                None => Err(MalError::TypeError),
            };
        } else {
            list.values()
        }
    } else if let Ok(vector) = to_quote.as_type::<MalVec>() {
        vector.values()
    } else {
        return quote(to_quote);
    };

    let mut qq = Vec::with_capacity(elems.len());
    for elem in elems {
        match elem.as_array() {
            Ok(arr) if !arr.is_empty() => {
                if arr[0].is_special("unquote") {
                    let result = match arr.get(1) {
                        Some(ast) => eval(ast.clone(), env)?,
                        None => return Err(MalError::TypeError),
                    };
                    qq.push(result);
                } else if arr[0].is_special("splice-unquote") {
                    let result = match arr.get(1) {
                        Some(ast) => eval(ast.clone(), env)?,
                        None => return Err(MalError::TypeError),
                    };
                    let result = result.as_array()?;
                    for item in result {
                        qq.push(item.clone());
                    }
                } else {
                    qq.push(elem.clone());
                }
            }
            _ => qq.push(elem.clone()),
        }
    }
    if to_quote.is::<MalList>() {
        Ok(Rc::from(MalList::from(qq)))
    } else {
        Ok(Rc::from(MalVec::from(qq)))
    }
}

#[builtin_func(name = "defmacro", special)]
pub fn defmacro_fn(symbol: &MalSymbol, ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let value = eval(ast.clone(), env)?;
    value.as_type::<MalClojure>()?.set_macro();
    env.set(symbol, value.clone());
    Ok(value)
}

pub fn is_macro_call(list: &MalList, env: &Rc<Env>) -> bool {
    if !list.is_empty() {
        let symbol = match list[0].as_type::<MalSymbol>() {
            Ok(s) => s,
            _ => return false,
        };
        match env.get(symbol) {
            Ok(maybe_clojure) => match maybe_clojure.as_type::<MalClojure>() {
                Ok(clojure) => clojure.is_macro(),
                Err(_) => false,
            },
            Err(_) => false,
        }
    } else {
        false
    }
}

pub fn macro_expand(mut ast: Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    while let Ok(call) = ast.as_type::<MalList>() {
        if !is_macro_call(call, env) {
            break;
        }
        let symbol = call[0].as_type()?;
        let lookup = env.get(symbol).unwrap();
        let macro_clojure = lookup.as_type::<MalClojure>().unwrap();
        let (new_ast, new_env) = macro_clojure.call(&call.values()[1..], env)?;
        ast = eval(new_ast, &new_env)?;
    }
    Ok(ast)
}

#[builtin_func(name = "try", symbol = "try*", special)]
pub fn try_fn(ast: &Rc<dyn MalType>, catch: Option<&Rc<dyn MalType>>, env: &Rc<Env>) -> MalResult {
    match eval(ast.clone(), env) {
        Ok(result) => Ok(result),
        Err(err) => {
            let exception = if let MalError::Exception(exception) = err {
                exception
            } else {
                Rc::from(MalException::from(err))
            };
            if let Some(catch) = catch {
                let catch = catch.as_type::<MalList>()?;
                if !catch.is_special("catch*") || catch.len() != 3 {
                    return Err(MalError::TypeError);
                }
                let symbol = catch[1].as_type()?;
                let outer = Env::with_outer(env.clone());
                outer.set(symbol, exception);
                eval(catch[2].clone(), &outer)
            } else {
                Ok(exception)
            }
        }
    }
}
