use std::rc::Rc;

use mal::{reader::Reader, types::MalType};
use rustyline::{error::ReadlineError, Editor};

fn read(input: String) -> Result<Rc<dyn MalType>, &'static str> {
    let mut reader = Reader::from(input).peekable();
    Reader::read_from(&mut reader)
}

fn eval(input: Rc<dyn MalType>) -> Result<Rc<dyn MalType>, &'static str> {
    Ok(input)
}

fn print(input: Rc<dyn MalType>) -> String {
    format!("{}", input)
}

fn rep(input: String) -> Result<String, &'static str> {
    let ast = read(input)?;
    let result = eval(ast)?;
    Ok(print(result))
}

fn main() {
    let mut editor = Editor::<()>::new();
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => {
                editor.add_history_entry(&line);
                match rep(line) {
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
