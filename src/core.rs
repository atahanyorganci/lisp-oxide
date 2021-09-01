use std::{fmt::Write, fs, rc::Rc};

use mal_derive::builtin_func;

use crate::{
    env::{self, Env},
    eval, read,
    types::{
        func::MalFuncPtr, MalAtom, MalBool, MalInt, MalList, MalNil, MalString, MalType, MalVec,
    },
    MalError, MalResult,
};

#[builtin_func(symbol = "+")]
pub fn add(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs + rhs))
}

#[builtin_func(symbol = "-")]
pub fn subtract(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs - rhs))
}

#[builtin_func(symbol = "*")]
pub fn multiply(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(lhs * rhs))
}

#[builtin_func(symbol = "/")]
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

#[builtin_func(symbol = "list?")]
pub fn is_list(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalList>())))
}

#[builtin_func(symbol = "empty?")]
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

#[builtin_func(symbol = "=")]
pub fn equal(lhs: &dyn MalType, rhs: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs.equal(rhs))))
}

#[builtin_func(symbol = "<")]
pub fn lt(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs < rhs)))
}

#[builtin_func(symbol = "<=")]
pub fn leq(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs <= rhs)))
}

#[builtin_func(symbol = ">")]
pub fn gt(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs > rhs)))
}

#[builtin_func(symbol = ">=")]
pub fn geq(lhs: &MalInt, rhs: &MalInt) -> MalResult {
    Ok(Rc::from(MalBool::from(lhs >= rhs)))
}

#[builtin_func(symbol = "pr-str")]
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

#[builtin_func(symbol = "read-string")]
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

#[builtin_func(symbol = "atom?")]
pub fn is_atom(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalAtom>())))
}

#[builtin_func]
pub fn deref(atom: &MalAtom) -> MalResult {
    Ok(atom.value())
}

#[builtin_func(symbol = "reset!")]
pub fn reset(atom: &MalAtom, new_value: &Rc<dyn MalType>) -> MalResult {
    atom.replace(new_value.clone());
    Ok(atom.value())
}

#[builtin_func(symbol = "swap!")]
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

#[builtin_func]
pub fn cons(elem: &Rc<dyn MalType>, list: &Rc<dyn MalType>) -> MalResult {
    let arr = list.as_array()?;
    let mut vec = Vec::with_capacity(arr.len());
    vec.push(elem.clone());
    for elem in arr {
        vec.push(elem.clone());
    }
    Ok(Rc::from(MalList::from(vec)))
}

#[builtin_func]
pub fn concat(elems: &[Rc<dyn MalType>]) -> MalResult {
    let mut capacity = 0;
    for elem in elems {
        capacity += elem.as_array()?.len();
    }

    let mut result = Vec::with_capacity(capacity);
    for elem in elems {
        let arr = elem.as_array()?;
        for item in arr {
            result.push(item.clone());
        }
    }

    Ok(Rc::from(MalList::from(result)))
}

#[builtin_func]
pub fn vec(list: &Rc<dyn MalType>) -> MalResult {
    Ok(Rc::from(MalVec::from(Vec::from(list.as_array()?))))
}
