use std::{convert::TryInto, fmt::Write, fs, rc::Rc};

use mal_derive::builtin_func;

use crate::{
    env::{self, Env},
    eval, read,
    types::{
        func::MalFuncPtr, MalAtom, MalBool, MalClojure, MalFunc, MalHashMap, MalInt, MalKeyword,
        MalList, MalNil, MalString, MalSymbol, MalType, MalVec,
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

#[builtin_func]
pub fn nth(arr: &Rc<dyn MalType>, idx: &MalInt) -> MalResult {
    let arr = arr.as_array()?;
    let idx: usize = match (*idx).try_into() {
        Ok(idx) => idx,
        Err(_) => return Err(MalError::TypeError),
    };
    match arr.get(idx) {
        Some(result) => Ok(result.clone()),
        None => Err(MalError::IndexOutOfRange),
    }
}

#[builtin_func]
pub fn first(list_or_vec: &Rc<dyn MalType>) -> MalResult {
    if let Ok(arr) = list_or_vec.as_array() {
        match arr.get(0) {
            Some(result) => Ok(result.clone()),
            None => Ok(MalNil::new()),
        }
    } else if list_or_vec.is::<MalNil>() {
        Ok(list_or_vec.clone())
    } else {
        Err(MalError::TypeError)
    }
}

#[builtin_func]
pub fn rest(list_or_vec: &Rc<dyn MalType>) -> MalResult {
    if list_or_vec.is::<MalNil>() {
        return Ok(Rc::from(MalList::new()));
    }
    let arr = list_or_vec.as_array()?;
    if arr.is_empty() {
        Ok(Rc::from(MalList::new()))
    } else {
        let r: Vec<_> = arr.iter().skip(1).cloned().collect();
        Ok(Rc::from(MalList::from(r)))
    }
}

#[builtin_func]
pub fn throw(value: &Rc<dyn MalType>) -> MalResult {
    Err(MalError::Exception(value.clone()))
}

#[builtin_func]
pub fn apply(func: &Rc<dyn MalType>, args: &[Rc<dyn MalType>], env: &Rc<Env>) -> MalResult {
    if args.is_empty() {
        return Err(MalError::TypeError);
    }
    let len = args.len();
    let regular_args = &args[0..len - 1];
    let list_args = match args.last().unwrap().as_array() {
        Ok(arr) => arr,
        Err(_) => return Err(MalError::TypeError),
    };
    let args: Vec<_> = regular_args.iter().chain(list_args).cloned().collect();
    if let Ok(func) = func.as_type::<MalFunc>() {
        func.call(&args, env)
    } else if let Ok(clojure) = func.as_type::<MalClojure>() {
        let (ast, env) = clojure.call(&args, env)?;
        eval(ast, &env)
    } else {
        Err(MalError::TypeError)
    }
}

#[builtin_func]
pub fn map(func: &Rc<dyn MalType>, args: &Rc<dyn MalType>, env: &Rc<Env>) -> MalResult {
    let arr = args.as_array()?;
    let len = arr.len();
    let mut result = Vec::with_capacity(len);

    if let Ok(func) = func.as_type::<MalFunc>() {
        for i in 0..len {
            result.push(func.call(&arr[i..i + 1], env)?);
        }
    } else if let Ok(clojure) = func.as_type::<MalClojure>() {
        for i in 0..len {
            let (ast, env) = clojure.call(&arr[i..i + 1], env)?;
            result.push(eval(ast, &env)?);
        }
    } else {
        return Err(MalError::TypeError);
    }
    Ok(Rc::from(MalList::from(result)))
}

#[builtin_func(symbol = "nil?")]
pub fn is_nil(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalNil>())))
}

#[builtin_func(symbol = "true?")]
pub fn is_true(obj: &dyn MalType) -> MalResult {
    match obj.as_type::<MalBool>() {
        Ok(b) => Ok(Rc::from(MalBool::from(b.value()))),
        Err(_) => Ok(Rc::from(MalBool::from(false))),
    }
}

#[builtin_func(symbol = "false?")]
pub fn is_false(obj: &dyn MalType) -> MalResult {
    match obj.as_type::<MalBool>() {
        Ok(b) => Ok(Rc::from(MalBool::from(!b.value()))),
        Err(_) => Ok(Rc::from(MalBool::from(false))),
    }
}

#[builtin_func(symbol = "symbol?")]
pub fn is_symbol(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalSymbol>())))
}

#[builtin_func]
pub fn symbol(string: &MalString) -> MalResult {
    Ok(Rc::from(MalSymbol::from(string.as_str())))
}

#[builtin_func(symbol = "keyword?")]
pub fn is_keyword(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalKeyword>())))
}

#[builtin_func]
pub fn keyword(arg: &Rc<dyn MalType>) -> MalResult {
    if arg.is::<MalKeyword>() {
        Ok(arg.clone())
    } else if let Ok(string) = arg.as_type::<MalString>() {
        Ok(Rc::from(MalKeyword::from(format!(":{}", string))))
    } else {
        Err(MalError::TypeError)
    }
}

#[builtin_func]
pub fn vector(args: &[Rc<dyn MalType>]) -> MalResult {
    Ok(Rc::from(MalVec::from(Vec::from(args))))
}

#[builtin_func(symbol = "vector?")]
pub fn is_vector(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalVec>())))
}

#[builtin_func(symbol = "sequential?")]
pub fn is_sequential(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.as_array().is_ok())))
}

#[builtin_func(symbol = "hash-map")]
pub fn hash_map(args: &[Rc<dyn MalType>]) -> MalResult {
    if args.len() % 2 != 0 {
        return Err(MalError::TypeError);
    }
    let items = args.iter().cloned();
    let map = MalHashMap::try_from_iter(items)?;
    Ok(Rc::from(map))
}

#[builtin_func(symbol = "map?")]
pub fn is_map(obj: &dyn MalType) -> MalResult {
    Ok(Rc::from(MalBool::from(obj.is::<MalHashMap>())))
}

#[builtin_func]
pub fn assoc(map: &MalHashMap, args: &[Rc<dyn MalType>]) -> MalResult {
    if args.len() % 2 != 0 {
        return Err(MalError::TypeError);
    }
    let result = map.insert(args.iter().cloned())?;
    Ok(Rc::from(result))
}

#[builtin_func]
pub fn dissoc(map: &MalHashMap, args: &[Rc<dyn MalType>]) -> MalResult {
    let result = map.remove(args.iter())?;
    Ok(Rc::from(result))
}

#[builtin_func]
pub fn get(map: &Rc<dyn MalType>, arg: &Rc<dyn MalType>) -> MalResult {
    if map.is::<MalNil>() {
        return Ok(MalNil::new());
    }
    let map = map.as_type::<MalHashMap>()?;
    let key = if let Ok(string) = arg.as_type::<MalString>() {
        &string.value
    } else if let Ok(keyword) = arg.as_type::<MalKeyword>() {
        &keyword.value
    } else {
        return Err(MalError::TypeError);
    };
    if let Some(value) = map.get(key) {
        Ok(value.clone())
    } else {
        Ok(MalNil::new())
    }
}

#[builtin_func(symbol = "contains?")]
pub fn contains(map: &MalHashMap, arg: &Rc<dyn MalType>) -> MalResult {
    let key = if let Ok(string) = arg.as_type::<MalString>() {
        &string.value
    } else if let Ok(keyword) = arg.as_type::<MalKeyword>() {
        &keyword.value
    } else {
        return Err(MalError::TypeError);
    };
    Ok(Rc::from(MalBool::from(map.contains(key))))
}

#[builtin_func]
pub fn keys(map: &MalHashMap) -> MalResult {
    let list: Vec<_> = map
        .keys()
        .map(|s| {
            let result: Rc<dyn MalType> = if s.starts_with(':') {
                Rc::from(MalKeyword::from(s.as_str()))
            } else {
                Rc::from(MalString::from(s.as_str()))
            };
            result
        })
        .collect();
    Ok(Rc::from(MalList::from(list)))
}

#[builtin_func]
pub fn vals(map: &MalHashMap) -> MalResult {
    let list: Vec<_> = map.values().cloned().collect();
    Ok(Rc::from(MalList::from(list)))
}
