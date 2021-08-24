use std::rc::Rc;

use mal::{env::Env, MalError};
use rustyline::{error::ReadlineError, Config, Editor};

fn rep(input: &str, environment: Rc<Env>) -> Result<String, MalError> {
    let ast = mal::read(input)?;
    let result = mal::eval(ast, environment)?;
    Ok(mal::print(result))
}

fn main() {
    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::<()>::with_config(config);

    let env = Rc::from(Env::default());
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => match rep(line.as_str(), env.clone()) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{}", err),
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}
