use std::{
    any::Any,
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{env::Env, eval, MalResult};

use super::{MalClojure, MalFunc, MalType};

pub struct MalAtom {
    value: RefCell<Rc<dyn MalType>>,
}

impl From<Rc<dyn MalType>> for MalAtom {
    fn from(value: Rc<dyn MalType>) -> Self {
        Self {
            value: RefCell::from(value),
        }
    }
}

impl MalAtom {
    pub fn value(&self) -> Rc<dyn MalType> {
        self.value.borrow().clone()
    }

    pub fn replace(&self, new_value: Rc<dyn MalType>) {
        self.value.replace(new_value);
    }

    pub fn update_with_fn(
        &self,
        func: &MalFunc,
        args: &[Rc<dyn MalType>],
        env: &Rc<Env>,
    ) -> MalResult {
        let args = self.update_args(args);
        let new_value = func.call(&args, env)?;
        self.value.replace(new_value.clone());
        Ok(new_value)
    }

    pub fn update_with_clojure(
        &self,
        clojure: &MalClojure,
        args: &[Rc<dyn MalType>],
        env: &Rc<Env>,
    ) -> MalResult {
        let args = self.update_args(args);
        let (new_ast, new_env) = clojure.call(&args, env)?;
        let new_value = eval(new_ast, &new_env)?;
        self.value.replace(new_value.clone());
        Ok(new_value)
    }

    fn update_args(&self, args: &[Rc<dyn MalType>]) -> Vec<Rc<dyn MalType>> {
        if !args.is_empty() {
            let mut updated = Vec::with_capacity(args.len() + 1);
            updated.push(self.value());
            for arg in args {
                updated.push(arg.clone());
            }
            updated
        } else {
            vec![self.value()]
        }
    }
}

impl Debug for MalAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(atom {:?})", self.value.borrow())
    }
}

impl Display for MalAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "(atom {})", self.value.borrow())
    }
}

impl MalType for MalAtom {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn equal(&self, rhs: &dyn MalType) -> bool {
        match rhs.as_type::<MalAtom>() {
            Ok(rhs) => self.value.borrow().equal(rhs.value.borrow().as_ref()),
            Err(_) => false,
        }
    }
}
