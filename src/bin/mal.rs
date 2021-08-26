use std::{borrow::Cow, fmt::Write, rc::Rc};

use mal::{
    env::Env,
    reader::{AtomKind, Reader, Token, Tokenizer},
};
use rustyline::{
    completion::Completer,
    error::ReadlineError,
    highlight::Highlighter,
    hint::Hinter,
    validate::{ValidationContext, ValidationResult, Validator},
    Config, Editor, Helper,
};

pub struct MalHelper {
    env: Rc<Env>,
}

impl From<Rc<Env>> for MalHelper {
    fn from(env: Rc<Env>) -> Self {
        Self { env }
    }
}

impl Completer for MalHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let tokenizer = Tokenizer::from(line);

        for token in tokenizer {
            if token.stop < pos {
                continue;
            }
            if let Token::Atom(atom) = token.as_token() {
                return Ok((token.start, self.env.starts_with(atom)));
            }
        }
        Err(ReadlineError::Eof)
    }
}

impl Hinter for MalHelper {
    type Hint = String;
}

impl Highlighter for MalHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let width = (line.len() as f64 * 1.5) as usize;
        let mut owned = String::with_capacity(width);

        let tokenizer = Tokenizer::from(line);
        for full_token in tokenizer {
            let token = full_token.as_token();
            match token {
                Token::TildeAt
                | Token::LeftSquare
                | Token::RightSquare
                | Token::LeftCurly
                | Token::RightCurly
                | Token::LeftParen
                | Token::RightParen
                | Token::Apostrophe
                | Token::BackTick
                | Token::Tilde
                | Token::Caret
                | Token::Space
                | Token::Newline
                | Token::CarriageReturn
                | Token::Tab
                | Token::Comma
                | Token::At => owned.write_fmt(format_args!("{}", token)).unwrap(),
                Token::String(string) => {
                    owned
                        .write_fmt(format_args!("\x1b[1;31m\"{}\"\x1b[0m", string))
                        .unwrap();
                }
                Token::Comment(comment) => {
                    owned
                        .write_fmt(format_args!("\x1b[1;32m{}\x1b[0m", comment))
                        .unwrap();
                }
                Token::Atom(atom) => match AtomKind::from(atom.as_str()) {
                    AtomKind::Bool => owned
                        .write_fmt(format_args!("\x1b[1;34m{}\x1b[0m", atom))
                        .unwrap(),
                    AtomKind::Keyword => owned.write_str(atom).unwrap(),
                    AtomKind::Symbol => owned
                        .write_fmt(format_args!("\x1b[1;96m{}\x1b[0m", atom))
                        .unwrap(),
                    AtomKind::Nil => owned
                        .write_fmt(format_args!("\x1b[1;34m{}\x1b[0m", atom))
                        .unwrap(),
                    AtomKind::Int => owned.write_str(atom).unwrap(),
                    AtomKind::Builtin => owned
                        .write_fmt(format_args!("\x1b[1;35m{}\x1b[0m", atom))
                        .unwrap(),
                },
                Token::IncompleteString(incomplete) => owned
                    .write_fmt(format_args!("\x1b[1;31m\"{}\x1b[0m", incomplete))
                    .unwrap(),
                Token::IncompleteEmptyString => owned.write_str("\x1b[1;31m\"\x1b[0m").unwrap(),
            }
        }
        Cow::Owned(owned)
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        true
    }
}

impl Validator for MalHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let input = ctx.input();
        let reader = Reader::from(input);

        let mut curly = 0;
        let mut square = 0;
        let mut paren = 0;
        for token in reader {
            match token {
                Token::LeftSquare => square += 1,
                Token::RightSquare => square -= 1,
                Token::LeftCurly => curly += 1,
                Token::RightCurly => curly -= 1,
                Token::LeftParen => paren += 1,
                Token::RightParen => paren -= 1,
                Token::TildeAt
                | Token::Apostrophe
                | Token::BackTick
                | Token::Tilde
                | Token::Caret
                | Token::At
                | Token::IncompleteString(_)
                | Token::Space
                | Token::Newline
                | Token::CarriageReturn
                | Token::Tab
                | Token::Comma
                | Token::IncompleteEmptyString
                | Token::String(_)
                | Token::Comment(_)
                | Token::Atom(_) => {}
            }
        }
        if curly != 0 || square != 0 || paren != 0 {
            Ok(ValidationResult::Incomplete)
        } else {
            Ok(ValidationResult::Valid(None))
        }
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}

impl Helper for MalHelper {}

fn main() {
    let config = Config::builder().auto_add_history(true).build();
    let mut editor = Editor::<MalHelper>::with_config(config);
    let env = Env::new();

    editor.set_helper(Some(MalHelper::from(env.clone())));

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
