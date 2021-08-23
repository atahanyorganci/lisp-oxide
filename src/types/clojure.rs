use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{env::Env, eval, MalError, MalResult};

use super::{MalSymbol, MalType};

#[derive(Debug)]
pub struct MalClojure {
    arg_symbols: Vec<Rc<dyn MalType>>,
    body: Rc<dyn MalType>,
    outer: Rc<Env>,
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

    fn equal(&self, _rhs: &dyn MalType) -> bool {
        todo!()
    }
}

impl MalClojure {
    pub fn try_new(args: &[Rc<dyn MalType>], body: Rc<dyn MalType>, outer: Rc<Env>) -> MalResult {
        for arg in args {
            if !arg.is::<MalSymbol>() {
                return Err(MalError::TypeError);
            }
        }
        let arg_symbols = Vec::from(args);
        Ok(Rc::from(Self {
            arg_symbols,
            body,
            outer,
        }))
    }
}

impl MalClojure {
    pub fn call(&self, arg_exprs: &[Rc<dyn MalType>], env: Rc<Env>) -> MalResult {
        if self.arg_symbols.len() != arg_exprs.len() {
            return Err(MalError::TypeError);
        }
        let current = Env::with_outer(self.outer.clone());
        for (smybol, expr) in self.arg_symbols.iter().zip(arg_exprs) {
            let value = eval(expr.clone(), env.clone())?;
            current.set(smybol, value)?;
        }
        eval(self.body.clone(), current)
    }
}
