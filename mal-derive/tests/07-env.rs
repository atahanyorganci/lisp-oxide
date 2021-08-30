use std::rc::Rc;

use mal::{
    env::{self, Env},
    types::{MalAtom, MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn swap(
    atom: &MalAtom,
    callable: &Rc<dyn MalType>,
    args: &[Rc<dyn MalType>],
    env: &Rc<env::Env>,
) -> MalResult {
    if let Ok(func) = callable.as_type() {
        atom.update_with_fn(func, args, env)
    } else if let Ok(clojure) = callable.as_type() {
        atom.update_with_clojure(clojure, args, env)
    } else {
        Err(MalError::TypeError)
    }
}

fn main() {
    let value: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![value];
    let env = Env::new();

    mal_swap(args.as_slice(), &env).unwrap_err();
}
