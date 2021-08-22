use std::{collections::HashMap, rc::Rc};

use env::Env;
use reader::Reader;
use types::{MalHashMap, MalList, MalType, MalVec};

use crate::types::MalFunc;

pub mod env;
pub mod reader;
pub mod types;

pub fn read(input: String) -> Result<Rc<dyn MalType>, &'static str> {
    let mut reader = Reader::from(input).peekable();
    Reader::read_from(&mut reader)
}

pub fn eval(ast: Rc<dyn MalType>, env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    if let Ok(list) = ast.as_type::<MalList>() {
        if list.is_empty() {
            return Ok(ast);
        }
        let new_list = eval_ast(ast, env)?;
        let values = new_list.as_type::<MalList>().unwrap().values();
        if let Ok(func) = values[0].as_type::<MalFunc>() {
            func.call(&values[1..], env)
        } else {
            Err("not a function")
        }
    } else {
        eval_ast(ast, env)
    }
}

pub fn print(input: Rc<dyn MalType>) -> String {
    format!("{}", input)
}

pub fn eval_ast(ast: Rc<dyn MalType>, env: &Env) -> Result<Rc<dyn MalType>, &'static str> {
    if let Ok(symbol) = ast.as_type() {
        match env.lookup(symbol) {
            Ok(value) => Ok(value),
            Err(_) => Err("404"),
        }
    } else if let Ok(list) = ast.as_type::<MalList>() {
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
    } else {
        Ok(ast)
    }
}
