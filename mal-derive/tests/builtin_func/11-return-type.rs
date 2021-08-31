use std::rc::Rc;

use mal::{
    env::Env,
    eval,
    types::{MalInt, MalType},
    MalError, MalResult,
};

use mal_derive::builtin_func;

#[builtin_func(name = "let")]
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
        let symbol = env_list[2 * i].clone();
        let value = eval(env_list[2 * i + 1].clone(), &new_env)?;
        new_env.set(&symbol, value.clone())?;
    }
    Ok((ast.clone(), new_env))
}

fn main() {}
