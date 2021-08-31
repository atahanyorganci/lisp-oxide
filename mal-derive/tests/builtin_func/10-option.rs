use std::rc::Rc;

use mal::{
    env::Env,
    eval,
    types::{MalInt, MalNil, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

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

fn main() {
    let cond: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let value: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![cond, value];
    let env = Env::new();

    mal_if(args.as_slice(), &env).unwrap();
}
