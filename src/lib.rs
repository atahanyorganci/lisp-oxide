use std::{collections::HashMap, fmt::Display, rc::Rc};

use env::{def_fn, let_fn, Env};
use reader::Reader;
use types::{MalHashMap, MalList, MalSymbol, MalType, MalVec};

use crate::types::MalFunc;

pub mod env;
pub mod reader;
pub mod types;

pub type MalResult = Result<Rc<dyn MalType>, MalError>;

pub enum MalError {
    NotCallable,
    NotFound(Rc<dyn MalType>),
    EOF,
    Unbalanced,
    TypeError,
    Unimplemented,
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
        }
    }
}

pub fn read(input: String) -> MalResult {
    let mut reader = Reader::from(input).peekable();
    Reader::read_from(&mut reader)
}

pub fn eval(ast: Rc<dyn MalType>, env: &mut Env) -> MalResult {
    if let Ok(list) = ast.as_type::<MalList>() {
        if list.is_empty() {
            return Ok(ast);
        } else if list.is_def() {
            def_fn(&list.values()[1..], env)
        } else if list.is_let() {
            let_fn(&list.values()[1..], env)
        } else {
            let new_list = eval_ast(ast, env)?;
            let values = new_list.as_type::<MalList>()?.values();
            if let Ok(func) = values[0].as_type::<MalFunc>() {
                func.call(&values[1..], env)
            } else {
                Err(MalError::NotCallable)
            }
        }
    } else {
        eval_ast(ast, env)
    }
}

pub fn print(input: Rc<dyn MalType>) -> String {
    format!("{}", input)
}

pub fn eval_ast(ast: Rc<dyn MalType>, env: &mut Env) -> MalResult {
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
