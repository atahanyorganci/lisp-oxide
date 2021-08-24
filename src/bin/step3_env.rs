use std::rc::Rc;

use mal::{env::Env, MalError};
use rustyline::{error::ReadlineError, Editor};

fn rep(input: &str, environment: Rc<Env>) -> Result<String, MalError> {
    let ast = mal::read(input)?;
    let result = mal::eval(ast, environment)?;
    Ok(mal::print(result))
}

fn main() {
    let mut editor = Editor::<()>::new();
    let env = Rc::from(Env::default());
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => {
                editor.add_history_entry(&line);
                match rep(line.as_str(), env.clone()) {
                    Ok(result) => println!("{}", result),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}
