use std::rc::Rc;

use mal::{
    env::Env,
    types::{MalInt, MalNil, MalType},
    MalResult,
};

use mal_derive::builtin_func;

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

fn main() {
    let value: Rc<dyn MalType> = Rc::from(MalInt::from(5));
    let args = vec![value];
    let env = Env::new();

    mal_println(args.as_slice(), &env).unwrap();
}
