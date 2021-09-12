use std::rc::Rc;

use mal_core::{
    env::Env,
    types::{func::MalFuncPtr, MalBool, MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn equal(lhs: &dyn MalType, rhs: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs.equal(rhs))))
}

fn main() {
    let lhs: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let rhs: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![lhs, rhs];
    let env = Env::new();

    mal_equal(args.as_slice(), &env).unwrap();
}
