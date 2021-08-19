use std::io::{self, Write};

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
    loop {
        let mut input = String::new();
        inputline();
        match io::stdin().read_line(&mut input) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                println!("{}", rep(input.trim_end()));
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn inputline() {
    print!("user> ");
    io::stdout().flush().unwrap();
}
