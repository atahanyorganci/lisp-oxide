use crate::{
    types::{
        MalBool, MalHashMap, MalInt, MalKeyword, MalList, MalNil, MalString, MalSymbol, MalType,
        MalVec,
    },
    MalError, MalResult,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    iter::Peekable,
    rc::Rc,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AtomKind {
    Bool,
    Keyword,
    Symbol,
    Nil,
    Int,
    Builtin,
}

impl From<&str> for AtomKind {
    fn from(atom: &str) -> Self {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new("^-?\\d+$").unwrap();
        }
        if INT_RE.is_match_at(atom, 0) {
            AtomKind::Int
        } else if atom.starts_with(':') {
            AtomKind::Keyword
        } else if atom == "true" || atom == "false" {
            AtomKind::Bool
        } else if atom == "nil" {
            AtomKind::Nil
        } else {
            match atom {
                "def!" | "let*" | "do" | "if" | "fn*" => AtomKind::Builtin,
                _ => AtomKind::Symbol,
            }
        }
    }
}

#[derive(Debug)]
pub struct Reader<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> From<&'a str> for Reader<'a> {
    fn from(input: &'a str) -> Self {
        let tokenizer = Tokenizer::from(input);
        Self { tokenizer }
    }
}

impl Iterator for Reader<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokenizer.next() {
            Some(full_token) => {
                let token = full_token.into();
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
                    | Token::At
                    | Token::String(_)
                    | Token::Atom(_) => Some(token),
                    Token::Comment(_)
                    | Token::IncompleteString(_)
                    | Token::Space
                    | Token::Newline
                    | Token::CarriageReturn
                    | Token::Tab
                    | Token::Comma
                    | Token::IncompleteEmptyString => self.next(),
                }
            }
            None => None,
        }
    }
}

impl Reader<'_> {
    pub fn is_err(&self) -> bool {
        self.tokenizer.error.is_some()
    }

    pub fn read_from(reader: &mut Peekable<Self>) -> MalResult {
        match reader.peek() {
            Some(Token::LeftParen) => Reader::read_list(reader),
            Some(Token::LeftSquare) => Reader::read_vec(reader),
            Some(Token::LeftCurly) => Reader::read_hashmap(reader),
            Some(Token::Apostrophe)
            | Some(Token::Tilde)
            | Some(Token::BackTick)
            | Some(Token::TildeAt) => Reader::read_quote(reader),
            Some(Token::At) => Reader::read_deref(reader),
            Some(Token::RightParen) | Some(Token::RightSquare) | Some(Token::RightCurly) => {
                Err(MalError::Unbalanced)
            }
            Some(Token::String(_)) | Some(Token::Atom(_)) => Reader::read_atom(reader),
            Some(Token::Caret) => Err(MalError::Unimplemented),
            None => Err(MalError::EOF),
            Some(Token::Comment(_))
            | Some(Token::IncompleteString(_))
            | Some(Token::Space)
            | Some(Token::Newline)
            | Some(Token::CarriageReturn)
            | Some(Token::Tab)
            | Some(Token::Comma)
            | Some(Token::IncompleteEmptyString) => unreachable!(),
        }
    }

    pub fn read_quote(reader: &mut Peekable<Self>) -> MalResult {
        let token = reader.next().unwrap();
        let symbol = match token {
            Token::TildeAt | Token::Apostrophe | Token::BackTick | Token::Tilde => {
                token.try_into().unwrap()
            }
            _ => panic!("Invalid token: {:?}", token),
        };
        let quoted = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, quoted])))
    }

    pub fn read_deref(reader: &mut Peekable<Self>) -> MalResult {
        assert_eq!(reader.next().unwrap(), Token::At);
        let symbol: Rc<dyn MalType> = Rc::from(MalSymbol::from("deref".to_string()));
        let derefed = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, derefed])))
    }

    pub fn read_list(reader: &mut Peekable<Self>) -> MalResult {
        let list = Reader::read_between(reader, Token::LeftParen, Token::RightParen)?;
        Ok(Rc::from(MalList::from(list)))
    }

    pub fn read_vec(reader: &mut Peekable<Self>) -> MalResult {
        let list = Reader::read_between(reader, Token::LeftSquare, Token::RightSquare)?;
        Ok(Rc::from(MalVec::from(list)))
    }

    pub fn read_hashmap(reader: &mut Peekable<Self>) -> MalResult {
        let list = Reader::read_between(reader, Token::LeftCurly, Token::RightCurly)?;
        Ok(Rc::from(MalHashMap::try_from(list)?))
    }

    pub fn read_between(
        reader: &mut Peekable<Self>,
        start: Token,
        stop: Token,
    ) -> Result<Vec<Rc<dyn MalType>>, MalError> {
        assert_eq!(reader.next().unwrap(), start);
        let mut list = Vec::new();
        loop {
            match reader.peek() {
                Some(token) if *token == stop => break,
                Some(_) => {
                    list.push(Reader::read_from(reader)?);
                }
                None => {
                    return Err(MalError::Unbalanced);
                }
            }
        }
        assert_eq!(reader.next().unwrap(), stop);
        Ok(list)
    }

    pub fn read_atom(reader: &mut Peekable<Self>) -> MalResult {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new("^-?\\d+$").unwrap();
        }
        match reader.next() {
            Some(Token::Atom(atom)) => {
                if INT_RE.is_match_at(&atom, 0) {
                    let value = i64::from_str(&atom).unwrap();
                    Ok(Rc::from(MalInt::from(value)))
                } else if atom.starts_with(':') {
                    Ok(Rc::from(MalKeyword::from(atom)))
                } else if atom == "true" {
                    Ok(Rc::from(MalBool::from(true)))
                } else if atom == "false" {
                    Ok(Rc::from(MalBool::from(false)))
                } else if atom == "nil" {
                    Ok(MalNil::new())
                } else {
                    Ok(Rc::from(MalSymbol::from(atom)))
                }
            }
            Some(Token::String(string)) => Ok(Rc::from(MalString::from(string))),
            _ => Err(MalError::TypeError),
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    index: usize,
    error: Option<TokenizationError>,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(input: &'a str) -> Self {
        Tokenizer {
            input,
            index: 0,
            error: None,
        }
    }
}

impl PartialEq<Token> for FullToken {
    fn eq(&self, other: &Token) -> bool {
        &self.token == other
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TokenizationError {
    UnbalancedString,
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
    IncompleteString(String),
    Space,
    Newline,
    CarriageReturn,
    Tab,
    Comma,
    IncompleteEmptyString,
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
            | Token::Atom(_) => return Err(()),
            Token::Comment(_)
            | Token::IncompleteString(_)
            | Token::Space
            | Token::Newline
            | Token::CarriageReturn
            | Token::Tab
            | Token::Comma
            | Token::IncompleteEmptyString => unreachable!(),
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
            Token::IncompleteString(string) => write!(f, "\"{}", string),
            Token::IncompleteEmptyString => write!(f, "\""),
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

impl Iterator for Tokenizer<'_> {
    type Item = FullToken;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        let nth = self.input.chars().nth(self.index);
        let token = match nth {
            Some(',') => {
                self.index += 1;
                Token::Comma
            }
            Some(' ') => {
                self.index += 1;
                Token::Space
            }
            Some('\n') => {
                self.index += 1;
                Token::Newline
            }
            Some('\r') => {
                self.index += 1;
                Token::CarriageReturn
            }
            Some('\t') => {
                self.index += 1;
                Token::Tab
            }
            Some('~') => {
                let peeked = self.input.chars().nth(self.index + 1);
                match peeked {
                    Some('@') => {
                        self.index += 2;
                        Token::TildeAt
                    }
                    Some(_) => {
                        self.index += 1;
                        Token::Tilde
                    }
                    None => {
                        self.index += 1;
                        Token::Tilde
                    }
                }
            }
            Some('[') => {
                self.index += 1;
                Token::LeftSquare
            }
            Some(']') => {
                self.index += 1;
                Token::RightSquare
            }
            Some('{') => {
                self.index += 1;
                Token::LeftCurly
            }
            Some('}') => {
                self.index += 1;
                Token::RightCurly
            }
            Some('(') => {
                self.index += 1;
                Token::LeftParen
            }
            Some(')') => {
                self.index += 1;
                Token::RightParen
            }
            Some('\'') => {
                self.index += 1;
                Token::Apostrophe
            }
            Some('`') => {
                self.index += 1;
                Token::BackTick
            }
            Some('^') => {
                self.index += 1;
                Token::Caret
            }
            Some('@') => {
                self.index += 1;
                Token::At
            }
            Some('"') => {
                self.index += 1;

                let mut remaining = self.input.get(self.index..).unwrap().chars().peekable();
                let mut string = String::new();
                let string_token = loop {
                    let ch = match remaining.next() {
                        Some(ch) => ch,
                        None => {
                            if string.is_empty() {
                                break Token::IncompleteEmptyString;
                            } else {
                                break Token::IncompleteString(string);
                            }
                        }
                    };

                    self.index += 1;
                    match ch {
                        '"' => {
                            break Token::String(string);
                        }
                        '\\' => match remaining.peek() {
                            Some('"') => {
                                remaining.next().unwrap();
                                string.push('"');
                                self.index += 1;
                            }
                            Some('n') => {
                                remaining.next().unwrap();
                                string.push('\n');
                                self.index += 1;
                            }
                            Some('\\') => {
                                remaining.next().unwrap();
                                string.push('\\');
                                self.index += 1;
                            }
                            Some(_) => string.push('\\'),
                            None => {
                                break Token::IncompleteString(string);
                            }
                        },
                        _ => string.push(ch),
                    }
                };

                if let Token::IncompleteString(_) = string_token {
                    self.error = Some(TokenizationError::UnbalancedString);
                }
                string_token
            }
            Some(';') => {
                let chars = self.input.get(self.index..).unwrap().chars();
                let mut result = String::new();
                for ch in chars {
                    if ch != '\n' {
                        self.index += 1;
                        result.push(ch);
                    } else {
                        break;
                    }
                }
                Token::Comment(result)
            }
            Some(_) => {
                let chars = self.input.get(self.index..).unwrap().chars();
                let mut result = String::new();
                for ch in chars {
                    if !is_special_char(ch) {
                        self.index += 1;
                        result.push(ch);
                    } else {
                        break;
                    }
                }
                Token::Atom(result)
            }
            None => return None,
        };
        Some(FullToken::new(token, start, self.index))
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{Reader, Token};

    use super::Tokenizer;

    #[test]
    fn dont_read_whitespace_and_commas() {
        let reader = Reader::from(" \t\r\n,");
        let result: Vec<_> = reader.collect();
        assert_eq!(result, vec![])
    }

    #[test]
    fn read_special_characters() {
        let reader = Reader::from("[]{}()'`~^@~@");
        let result: Vec<_> = reader.collect();
        assert_eq!(
            result,
            vec![
                Token::LeftSquare,
                Token::RightSquare,
                Token::LeftCurly,
                Token::RightCurly,
                Token::LeftParen,
                Token::RightParen,
                Token::Apostrophe,
                Token::BackTick,
                Token::Tilde,
                Token::Caret,
                Token::At,
                Token::TildeAt
            ]
        )
    }

    #[test]
    fn read_strings() {
        let reader = Reader::from("\"one\" \"two\" \"three\"");
        let result: Vec<_> = reader.collect();
        assert_eq!(
            result,
            vec![
                Token::String(String::from("one")),
                Token::String(String::from("two")),
                Token::String(String::from("three")),
            ]
        )
    }

    #[test]
    fn read_strings_with_escape_sequences() {
        let reader = Reader::from(r#"backslash "\\" double-quote "\"" newline "\n" "#);
        let result: Vec<_> = reader.collect();
        assert_eq!(
            result,
            vec![
                Token::Atom(String::from("backslash")),
                Token::String(String::from("\\")),
                Token::Atom(String::from("double-quote")),
                Token::String(String::from("\"")),
                Token::Atom(String::from("newline")),
                Token::String(String::from("\n")),
            ]
        )
    }

    #[test]
    fn dont_read_unbalanced_strings() {
        let reader = Reader::from("\"unbalanced\" \"strings");
        let result: Vec<_> = reader.collect();
        assert_eq!(result, vec![Token::String(String::from("unbalanced")),]);
    }

    #[test]
    fn read_symbols() {
        let reader = Reader::from("first second third");
        let result: Vec<_> = reader.collect();
        assert_eq!(
            result,
            vec![
                Token::Atom(String::from("first")),
                Token::Atom(String::from("second")),
                Token::Atom(String::from("third")),
            ]
        )
    }

    #[test]
    fn read_comments() {
        let reader = Reader::from("atom ; This is comment");
        let result: Vec<_> = reader.collect();
        assert_eq!(result, vec![Token::Atom(String::from("atom")),])
    }

    #[test]
    fn tokeize_whitspace_and_commans() {
        let tokenizer = Tokenizer::from(" \t\r\n");
        let result: Vec<_> = tokenizer.collect();
        assert_eq!(
            result,
            vec![
                Token::Space,
                Token::Tab,
                Token::CarriageReturn,
                Token::Newline
            ]
        );
    }

    #[test]
    fn tokeize_incomplete_strings() {
        let tokenizer = Tokenizer::from(r#""string""incomplete"#);
        let result: Vec<_> = tokenizer.collect();
        assert_eq!(
            result,
            vec![
                Token::String("string".to_string()),
                Token::IncompleteString("incomplete".to_string())
            ]
        );
    }

    #[test]
    fn tokeize_incomplete_empty_strings() {
        let tokenizer = Tokenizer::from("\"");
        let result: Vec<_> = tokenizer.collect();
        assert_eq!(result, vec![Token::IncompleteEmptyString]);
    }

    #[test]
    fn tokeize_comments_strings() {
        let tokenizer = Tokenizer::from("bruh ; This is a comment");
        let result: Vec<_> = tokenizer.collect();
        assert_eq!(
            result,
            vec![
                Token::Atom("bruh".to_string()),
                Token::Space,
                Token::Comment("; This is a comment".to_string()),
            ]
        );
    }
}
