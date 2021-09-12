use std::rc::Rc;

use mal_core::{
    env::Env,
    types::{func::MalFuncPtr, MalAtom, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn swap(atom: &MalAtom, callable: &Rc<dyn MalType>, args: &[Rc<dyn MalType>]) -> MalResult {
    let env = Env::new();
    if let Ok(func) = callable.as_type() {
        atom.update_with_fn(func, args, &env)
    } else if let Ok(clojure) = callable.as_type() {
        atom.update_with_clojure(clojure, args, &env)
    } else {
        Err(MalError::TypeError)
    }
}

fn main() {
    let args = vec![];
    let env = Env::new();
    mal_swap(args.as_slice(), &env).unwrap_err();
}
