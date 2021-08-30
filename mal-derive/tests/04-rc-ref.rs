use std::rc::Rc;

use mal::{
    env::Env,
    types::{MalAtom, MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func]
pub fn atom(value: &Rc<dyn MalType>) -> MalResult {
    Ok(Rc::from(MalAtom::from(value.clone())))
}

fn main() {
    let value: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![value];
    let env = Env::new();

    mal_atom(args.as_slice(), &env).unwrap();
}
