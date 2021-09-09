use std::{
    any::Any,
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{env::Env, eval, MalError, MalResult};

use super::{MalList, MalSymbol, MalType};

pub struct MalClojure {
    arg_symbols: Vec<Rc<dyn MalType>>,
    body: Rc<dyn MalType>,
    outer: Rc<Env>,
    is_macro: RefCell<bool>,
}

impl Debug for MalClojure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self.is_macro.borrow() {
            write!(f, "#<macro>")
        } else {
            write!(f, "#<function>")
        }
    }
}

impl Display for MalClojure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self.is_macro.borrow() {
            write!(f, "#<macro>")
        } else {
            write!(f, "#<function>")
        }
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
            is_macro: false.into(),
        }))
    }

    pub fn set_macro(&self) {
        *self.is_macro.borrow_mut() = true;
    }

    pub fn is_macro(&self) -> bool {
        *self.is_macro.borrow()
    }
}

impl MalClojure {
    pub fn call(
        &self,
        arg_exprs: &[Rc<dyn MalType>],
        env: &Rc<Env>,
    ) -> Result<(Rc<dyn MalType>, Rc<Env>), MalError> {
        let current = Env::with_outer(self.outer.clone());

        for i in 0..self.arg_symbols.len() {
            let symbol = match self.arg_symbols.get(i) {
                Some(symbol) => symbol.as_type::<MalSymbol>()?,
                None => return Err(MalError::TypeError),
            };
            if symbol == "&" {
                // If current symbol is `&` then next symbol should capture rest of expressions as list
                let symbol = match self.arg_symbols.get(i + 1) {
                    Some(symbol) => symbol.as_type::<MalSymbol>()?,
                    None => return Err(MalError::TypeError),
                };
                let value = if self.is_macro() {
                    let variadic: MalList = arg_exprs[i..].iter().collect();
                    Rc::from(variadic)
                } else {
                    MalClojure::get_variadic_args(&arg_exprs[i..], env)?
                };
                current.set(symbol, value);
                break;
            } else {
                let value = match arg_exprs.get(i) {
                    Some(expr) => expr.clone(),
                    None => return Err(MalError::TypeError),
                };
                current.set(symbol, value);
            }
        }
        Ok((self.body.clone(), current))
    }

    fn get_variadic_args(slice: &[Rc<dyn MalType>], env: &Rc<Env>) -> MalResult {
        let mut vector = Vec::with_capacity(slice.len());
        for expr in slice {
            vector.push(eval(expr.clone(), env)?);
        }
        Ok(Rc::from(MalList::from(vector)))
    }
}
