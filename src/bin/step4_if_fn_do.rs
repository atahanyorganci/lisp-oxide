use mal::env::Env;
use rustyline::{error::ReadlineError, Config, Editor};

fn main() {
    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::<()>::with_config(config);

    let env = Env::new();
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => match mal::rep(line.as_str(), env.clone()) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{}", err),
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}
