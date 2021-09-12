use std::rc::Rc;

use mal_core::{
    env::Env,
    eval,
    types::{func::MalFuncPtr, MalInt, MalNil, MalSymbol, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func(symbol = "/")]
pub fn divide(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs / rhs))
}

#[builtin_func]
pub fn prn(args: &[Rc<dyn MalType>]) -> MalResult {
    if !args.is_empty() {
        print!("{:?}", args[0]);
        for arg in &args[1..] {
            print!(" {:?}", arg);
        }
    }
    println!();
    Ok(MalNil::new())
}

#[builtin_func(name = "def", special)]
pub fn def_fn(symbol: &MalSymbol, ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let value = eval(ast.clone(), env)?;
    env.set(symbol, value.clone());
    Ok(value)
}

fn main() {}
