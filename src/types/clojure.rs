use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{env::Env, eval, MalError, MalResult};

use super::{MalList, MalSymbol, MalType};

pub struct MalClojure {
    arg_symbols: Vec<Rc<dyn MalType>>,
    body: Rc<dyn MalType>,
    outer: Rc<Env>,
}

impl Debug for MalClojure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<function>")
    }
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
        let current = Env::with_outer(self.outer.clone());

        for i in 0..self.arg_symbols.len() {
            let symbol = match self.arg_symbols.get(i) {
                Some(symbol) => symbol,
                None => return Err(MalError::TypeError),
            };
            if symbol.as_type::<MalSymbol>()? == "&" {
                // If current symbol is `&` then next symbol should capture rest of expressions as list
                let symbol = match self.arg_symbols.get(i + 1) {
                    Some(symbol) => symbol,
                    None => return Err(MalError::TypeError),
                };
                let value = MalClojure::eval_slice(&arg_exprs[i..], env.clone())?;
                current.set(symbol, value)?;
                break;
            } else {
                let expr = match arg_exprs.get(i) {
                    Some(expr) => expr,
                    None => return Err(MalError::TypeError),
                };
                let value = eval(expr.clone(), env.clone())?;
                current.set(symbol, value)?;
            }
        }
        eval(self.body.clone(), current)
    }

    fn eval_slice(slice: &[Rc<dyn MalType>], env: Rc<Env>) -> MalResult {
        let mut vector = Vec::with_capacity(slice.len());
        for expr in slice {
            vector.push(eval(expr.clone(), env.clone())?);
        }
        Ok(Rc::from(MalList::from(vector)))
    }
}
