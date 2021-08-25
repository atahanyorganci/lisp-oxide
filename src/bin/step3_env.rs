use mal::env::Env;
use rustyline::{error::ReadlineError, Editor};

fn main() {
    let mut editor = Editor::<()>::new();
    let env = Env::new();
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => {
                editor.add_history_entry(&line);
                match mal::rep(line.as_str(), &env) {
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
