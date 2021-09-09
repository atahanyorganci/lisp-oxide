use std::{env, rc::Rc};

use mal::{env::Env, rep};
use rustyline::{error::ReadlineError, Config, Editor};

fn print_startup_header(env: &Rc<Env>) {
    let line = r#"(println (str "Mal [" *host-language* "]"))"#;
    mal::rep(line, env).unwrap();
}

fn repl(env: Rc<Env>) {
    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::<()>::with_config(config);

    print_startup_header(&env);
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => match mal::rep(line.as_str(), &env) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{}", err),
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}

fn main() {
    let env = Env::new();
    let mut args = env::args().skip(1);
    if let Some(filename) = args.next() {
        let to_exec = format!("(load-file \"{}\")", filename);
        if let Err(err) = rep(to_exec.as_str(), &env) {
            eprintln!("{}", err);
        }
    } else {
        repl(env);
    }
}
