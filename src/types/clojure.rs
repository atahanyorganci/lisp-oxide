use std::{any::Any, fmt::Display, rc::Rc};

use crate::{env::Env, eval, MalError, MalResult};

use super::{MalSymbol, MalType};

#[derive(Debug)]
pub struct MalClojure {
    args: Vec<Rc<dyn MalType>>,
    body: Rc<dyn MalType>,
}

impl Display for MalClojure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function>")
    }
}

impl MalType for MalClojure {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MalClojure {
    pub fn try_new(args: &[Rc<dyn MalType>], body: Rc<dyn MalType>) -> MalResult {
        for arg in args {
            if !arg.is::<MalSymbol>() {
                return Err(MalError::TypeError);
            }
        }
        let args = Vec::from(args);
        Ok(Rc::from(Self { args, body }))
    }
}

impl MalClojure {
    pub fn call(&self, args: &[Rc<dyn MalType>], env: &mut Env) -> MalResult {
        if self.args.len() != args.len() {
            return Err(MalError::TypeError);
        }

        let mut values = Vec::with_capacity(args.len());
        for arg in args {
            values.push(eval(arg.clone(), env)?);
        }

        env.push_and_init(self.args.as_slice(), values.as_slice())?;
        let result = eval(self.body.clone(), env);
        env.pop();
        result
    }
}
