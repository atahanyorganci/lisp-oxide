use std::{convert::TryInto, fmt::Display, rc::Rc};

use crate::types::{MalSymbol, MalType};

impl PartialEq<Token> for FullToken {
    fn eq(&self, other: &Token) -> bool {
        &self.token == other
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    TildeAt,
    LeftSquare,
    RightSquare,
    LeftCurly,
    RightCurly,
    LeftParen,
    RightParen,
    Apostrophe,
    BackTick,
    Tilde,
    Caret,
    At,
    Space,
    Newline,
    CarriageReturn,
    Tab,
    Comma,
    String(String),
    Comment(String),
    Atom(String),
}

impl TryInto<Rc<dyn MalType>> for Token {
    type Error = ();

    fn try_into(self) -> Result<Rc<dyn MalType>, Self::Error> {
        let symbol = match self {
            Token::TildeAt => "splice-unquote",
            Token::Apostrophe => "quote",
            Token::BackTick => "quasiquote",
            Token::Tilde => "unquote",
            Token::Caret => "with-meta",
            Token::At => "deref",
            Token::LeftSquare
            | Token::RightSquare
            | Token::LeftCurly
            | Token::RightCurly
            | Token::LeftParen
            | Token::RightParen
            | Token::String(_)
            | Token::Comment(_)
            | Token::Space
            | Token::Newline
            | Token::CarriageReturn
            | Token::Tab
            | Token::Comma
            | Token::Atom(_) => return Err(()),
        };
        Ok(Rc::from(MalSymbol::from(symbol.to_string())))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullToken {
    token: Token,
    pub start: usize,
    pub stop: usize,
}

impl From<FullToken> for Token {
    fn from(full_token: FullToken) -> Self {
        full_token.token
    }
}

impl FullToken {
    pub fn new(token: Token, start: usize, stop: usize) -> Self {
        Self { token, start, stop }
    }

    pub fn as_token(&self) -> &Token {
        &self.token
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::TildeAt => write!(f, "~@"),
            Token::LeftSquare => write!(f, "["),
            Token::RightSquare => write!(f, "]"),
            Token::LeftCurly => write!(f, "{{"),
            Token::RightCurly => write!(f, "}}"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Apostrophe => write!(f, "'"),
            Token::BackTick => write!(f, "`"),
            Token::Tilde => write!(f, "~"),
            Token::Caret => write!(f, "^"),
            Token::At => write!(f, "@"),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::Comment(comment) => write!(f, ";{}", comment),
            Token::Atom(atom) => write!(f, "{}", atom),
            Token::Space => write!(f, " "),
            Token::Newline => writeln!(f),
            Token::CarriageReturn => write!(f, "\r"),
            Token::Tab => write!(f, "\t"),
            Token::Comma => write!(f, ","),
        }
    }
}

pub fn is_special_char(ch: char) -> bool {
    match ch {
        '~' | '[' | ']' | '{' | '}' | '(' | ')' | '\'' | '`' | '"' | ',' | ';' => true,
        _ if ch.is_whitespace() => true,
        _ => false,
    }
}
