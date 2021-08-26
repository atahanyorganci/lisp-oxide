use std::{fmt::Write, fs, rc::Rc};

use crate::{
    env::Env,
    eval, read,
    types::{MalBool, MalInt, MalList, MalNil, MalString, MalType},
    MalError, MalResult,
};

pub fn add(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs + rhs))
}

pub fn subtract(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs - rhs))
}

pub fn multiply(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs * rhs))
}

pub fn divide(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(lhs / rhs))
}

pub fn prn(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    if !args.is_empty() {
        print!("{:?}", args[0]);
        for arg in &args[1..] {
            print!(" {:?}", arg);
        }
    }
    println!();
    Ok(MalNil::new())
}

pub fn println_fn(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    if !args.is_empty() {
        print!("{}", args[0]);
        for arg in &args[1..] {
            print!(" {}", arg);
        }
    }
    println!();
    Ok(MalNil::new())
}

pub fn list(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    Ok(Rc::from(MalList::from(Vec::from(args))))
}

pub fn is_list(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    Ok(Rc::from(MalBool::from(args[0].is::<MalList>())))
}

pub fn is_empty(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let value = match args[0].as_array() {
        Ok(arr) => arr.is_empty(),
        Err(_) => true,
    };
    Ok(Rc::from(MalBool::from(value)))
}

pub fn count(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let value = match args[0].as_array() {
        Ok(arr) => arr.len() as i64,
        Err(_) => 0,
    };
    Ok(Rc::from(MalInt::from(value)))
}

pub fn equal(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    if args.len() != 2 {
        return Err(MalError::TypeError);
    }
    let rhs = args[0].as_ref();
    let lhs = args[1].as_ref();
    Ok(Rc::from(MalBool::from(lhs.equal(rhs))))
}

pub fn lt(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs < rhs)))
}

pub fn leq(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs <= rhs)))
}

pub fn gt(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs > rhs)))
}

pub fn geq(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let lhs = args[0].as_type::<MalInt>()?;
    let rhs = args[1].as_type::<MalInt>()?;
    Ok(Rc::from(MalBool::from(lhs >= rhs)))
}

pub fn pr_str(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let mut string = String::new();
    if !args.is_empty() {
        string.write_fmt(format_args!("{:?}", &args[0])).unwrap();
        for arg in &args[1..] {
            string.write_fmt(format_args!(" {:?}", arg)).unwrap();
        }
    }
    Ok(Rc::from(MalString::from(string)))
}

pub fn str_fn(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    let mut string = String::new();
    for arg in args {
        string.write_str(&arg.to_string()).unwrap();
    }
    Ok(Rc::from(MalString::from(string)))
}

pub fn read_string(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    if args.is_empty() {
        return Err(MalError::TypeError);
    }
    let result = match args[0].as_type::<MalString>() {
        Ok(string) => read(string.as_str())?,
        Err(_) => return Err(MalError::TypeError),
    };
    Ok(result)
}

pub fn slurp(args: &[Rc<dyn MalType>], _env: &Rc<Env>) -> MalResult {
    if args.is_empty() {
        return Err(MalError::TypeError);
    }
    match args[0].as_type::<MalString>() {
        Ok(string) => match fs::read_to_string(string.as_str()) {
            Ok(string) => Ok(Rc::from(MalString::from(string))),
            Err(_) => Err(MalError::IOError),
        },
        Err(_) => Err(MalError::TypeError),
    }
}

pub fn load_file(args: &[Rc<dyn MalType>], env: &Rc<Env>) -> MalResult {
    if args.is_empty() {
        return Err(MalError::TypeError);
    }
    let input = match args[0].as_type::<MalString>() {
        Ok(string) => match fs::read_to_string(string.as_str()) {
            Ok(string) => format!("(do {}\n nil)", &string),
            Err(_) => return Err(MalError::IOError),
        },
        Err(_) => return Err(MalError::TypeError),
    };
    let ast = read(input.as_str())?;
    eval(ast, env)
}
