use std::{
    any::Any,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::env::Env;

use super::MalType;

pub type MalFuncPtr = dyn Fn(&[Rc<dyn MalType>], &mut Env) -> Result<Rc<dyn MalType>, &'static str>;

pub struct MalFunc {
    name: &'static str,
    ptr: &'static MalFuncPtr,
}

impl Debug for MalFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for MalFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl MalType for MalFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl MalFunc {
    pub fn new(name: &'static str, ptr: &'static MalFuncPtr) -> Self {
        Self { name, ptr }
    }

    pub fn call(
        &self,
        args: &[Rc<dyn MalType>],
        env: &mut Env,
    ) -> Result<Rc<dyn MalType>, &'static str> {
        let func = self.ptr;
        func(args, env)
    }
}
