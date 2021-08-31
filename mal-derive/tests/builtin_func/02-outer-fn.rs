use std::rc::Rc;

use mal::{
    env::Env,
    types::{MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn add(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs + rhs))
}

fn main() {
    let lhs: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let rhs: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![lhs, rhs];
    let env = Env::new();

    mal_add(args.as_slice(), &env).unwrap();
}
