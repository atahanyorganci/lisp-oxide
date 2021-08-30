use std::{fmt::Write, fs, rc::Rc};

use mal_derive::builtin_func;

use crate::{
    env::{self, Env},
    eval, read,
    types::{MalAtom, MalBool, MalInt, MalList, MalNil, MalString, MalType},
    MalError, MalResult,
};

#[builtin_func]
pub fn add(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs + rhs))
}

#[builtin_func]
pub fn subtract(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs - rhs))
}

#[builtin_func]
pub fn multiply(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs * rhs))
}

#[builtin_func]
pub fn divide(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs / rhs))
}

#[builtin_func]
pub fn prn(args: &[Rc<dyn MalType>]) -> MalResult {
    if !args.is_empty() {
        print!("{:?}", args[0]);
        for arg in &args[1..] {
            print!(" {:?}", arg);
        }
    }
    println!();
    Ok(MalNil::new())
}

#[builtin_func(name = "println")]
pub fn println_fn(args: &[Rc<dyn MalType>]) -> MalResult {
    if !args.is_empty() {
        print!("{}", args[0]);
        for arg in &args[1..] {
            print!(" {}", arg);
        }
    }
    println!();
    Ok(MalNil::new())
}

#[builtin_func]
pub fn list(args: &[Rc<dyn MalType>]) -> MalResult {
    Ok(Rc::from(MalList::from(Vec::from(args))))
}

#[builtin_func]
pub fn is_list(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalList>())))
}

#[builtin_func]
pub fn is_empty(obj: &dyn MalType) -> MalResult {
    let value = match obj.as_array() {
        Ok(arr) => arr.is_empty(),
        Err(_) => true,
    };
    Ok(Rc::from(MalBool::from(value)))
}

#[builtin_func]
pub fn count(obj: &dyn MalType) -> MalResult {
    let value = match obj.as_array() {
        Ok(arr) => arr.len() as i64,
        Err(_) => 0,
    };
    Ok(Rc::from(MalInt::from(value)))
}

#[builtin_func]
pub fn equal(lhs: &dyn MalType, rhs: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs.equal(rhs))))
}

#[builtin_func]
pub fn lt(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs < rhs)))
}

#[builtin_func]
pub fn leq(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs <= rhs)))
}

#[builtin_func]
pub fn gt(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs > rhs)))
}

#[builtin_func]
pub fn geq(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs >= rhs)))
}

#[builtin_func]
pub fn pr_str(args: &[Rc<dyn MalType>]) -> MalResult {
    let mut string = String::new();
    if !args.is_empty() {
        string.write_fmt(format_args!("{:?}", &args[0])).unwrap();
        for arg in &args[1..] {
            string.write_fmt(format_args!(" {:?}", arg)).unwrap();
        }
    }
    Ok(Rc::from(MalString::from(string)))
}

#[builtin_func(name = "str")]
pub fn str_fn(args: &[Rc<dyn MalType>]) -> MalResult {
    let mut string = String::new();
    for arg in args {
        string.write_str(&arg.to_string()).unwrap();
    }
    Ok(Rc::from(MalString::from(string)))
}

#[builtin_func]
pub fn read_string(string: &MalString) -> MalResult {
    read(string.as_str())
}

#[builtin_func]
pub fn slurp(path: &MalString) -> MalResult {
    match fs::read_to_string(path.as_str()) {
        Ok(string) => Ok(Rc::from(MalString::from(string))),
        Err(_) => Err(MalError::IOError),
    }
}

#[builtin_func]
pub fn atom(value: &Rc<dyn MalType>) -> MalResult {
    Ok(Rc::from(MalAtom::from(value.clone())))
}

#[builtin_func]
pub fn is_atom(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalAtom>())))
}

#[builtin_func]
pub fn deref(atom: &MalAtom) -> MalResult {
    Ok(atom.value())
}

#[builtin_func]
pub fn reset(atom: &MalAtom, new_value: &Rc<dyn MalType>) -> MalResult {
    atom.replace(new_value.clone());
    Ok(atom.value())
}

#[builtin_func(name = "swap")]
pub fn swap(
    atom: &MalAtom,
    callable: &Rc<dyn MalType>,
    args: &[Rc<dyn MalType>],
    env: &Rc<env::Env>,
) -> MalResult {
    if let Ok(func) = callable.as_type() {
        atom.update_with_fn(func, args, env)
    } else if let Ok(clojure) = callable.as_type() {
        atom.update_with_clojure(clojure, args, env)
    } else {
        Err(MalError::TypeError)
    }
}

#[builtin_func(name = "eval")]
pub fn eval_fn(ast: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let env = env::global(env);
    eval(ast.clone(), env)
}
