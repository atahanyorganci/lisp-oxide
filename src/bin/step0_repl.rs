use rustyline::{error::ReadlineError, Editor};

fn read(input: &str) -> &str {
    input
}

fn eval(input: &str) -> &str {
    input
}

fn print(input: &str) -> &str {
    input
}

fn rep(input: &str) -> &str {
    let ast = read(input);
    let result = eval(ast);
    print(result)
}

fn main() {
    let mut editor = Editor::<()>::new();
    loop {
        let readline = editor.readline("user> ");
        match readline {
            Ok(line) => {
                println!("{}", rep(line.trim_end()));
                editor.add_history_entry(line);
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(err) => eprintln!("Unexpected error encountered {}.", err),
        }
    }
}
