use std::rc::Rc;

use mal_core::{env::Env, eval, types::MalType, MalError};

use mal_derive::builtin_func;

#[builtin_func(name = "let", special)]
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
        let symbol = env_list[2 * i].as_type()?;
        let value = eval(env_list[2 * i + 1].clone(), &new_env)?;
        new_env.set(symbol, value.clone());
    }
    Ok((ast.clone(), new_env))
}

fn main() {}
