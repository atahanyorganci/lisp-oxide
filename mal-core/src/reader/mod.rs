use crate::types::{
    MalBool, MalHashMap, MalInt, MalKeyword, MalList, MalNil, MalString, MalSymbol, MalType, MalVec,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    convert::{TryFrom, TryInto},
    iter::Peekable,
    rc::Rc,
    str::FromStr,
};
use thiserror::Error;

pub mod token;
pub mod tokenizer;

pub use token::{FullToken, Token};
pub use tokenizer::Tokenizer;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    #[error("Expected matching '\"'.")]
    UnbalancedEmptyString,
    #[error("Expected matching '\"' for `\"{0}`.")]
    UnbalancedString(String),
    #[error("Expected matching '['.")]
    UnbalancedList,
    #[error("Expected matching ']'.")]
    UnbalancedVec,
    #[error("Expected matching '}}'.")]
    UnbalancedMap,
    #[error("Unexpected token {0}.")]
    UnexpectedToken(Token),
    #[error("Reached end of input")]
    EOF,
}

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
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokenizer.next() {
            Some(Ok(full_token)) => {
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
                    | Token::Atom(_) => Some(Ok(token)),
                    Token::Comment(_)
                    | Token::Space
                    | Token::Newline
                    | Token::CarriageReturn
                    | Token::Tab
                    | Token::Comma => self.next(),
                }
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

pub type ReaderResult = Result<Rc<dyn MalType>, ParseError>;

impl Reader<'_> {
    pub fn read_from(reader: &mut Peekable<Self>) -> ReaderResult {
        let token = match reader.peek() {
            Some(Ok(token)) => token,
            Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
            None => return Err(ParseError::EOF),
        };

        match token {
            Token::LeftParen => Reader::read_list(reader),
            Token::LeftSquare => Reader::read_vec(reader),
            Token::LeftCurly => Reader::read_hashmap(reader),
            Token::Apostrophe | Token::Tilde | Token::BackTick | Token::TildeAt => {
                Reader::read_quote(reader)
            }
            Token::At => Reader::read_deref(reader),
            Token::RightParen => Err(ParseError::UnbalancedList),
            Token::RightSquare => Err(ParseError::UnbalancedVec),
            Token::RightCurly => Err(ParseError::UnbalancedMap),
            Token::String(_) | Token::Atom(_) => Reader::read_atom(reader),
            Token::Caret => unimplemented!(),
            Token::Comment(_)
            | Token::Space
            | Token::Newline
            | Token::CarriageReturn
            | Token::Tab
            | Token::Comma => unreachable!(),
        }
    }

    fn read_quote(reader: &mut Peekable<Self>) -> ReaderResult {
        let token = reader.next().unwrap()?;
        let symbol = match token {
            Token::TildeAt | Token::Apostrophe | Token::BackTick | Token::Tilde => {
                token.try_into().unwrap()
            }
            _ => panic!("Invalid token: {:?}", token),
        };
        let quoted = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, quoted])))
    }

    fn read_deref(reader: &mut Peekable<Self>) -> ReaderResult {
        assert_eq!(reader.next().unwrap().unwrap(), Token::At);
        let symbol: Rc<dyn MalType> = Rc::from(MalSymbol::from("deref".to_string()));
        let derefed = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, derefed])))
    }

    fn read_list(reader: &mut Peekable<Self>) -> ReaderResult {
        let list = Reader::read_between(reader, Token::LeftParen, Token::RightParen)?;
        Ok(Rc::from(MalList::from(list)))
    }

    fn read_vec(reader: &mut Peekable<Self>) -> ReaderResult {
        let list = Reader::read_between(reader, Token::LeftSquare, Token::RightSquare)?;
        Ok(Rc::from(MalVec::from(list)))
    }

    fn read_hashmap(reader: &mut Peekable<Self>) -> ReaderResult {
        let list = Reader::read_between(reader, Token::LeftCurly, Token::RightCurly)?;
        match MalHashMap::try_from(list) {
            Ok(map) => Ok(Rc::from(map)),
            Err(_) => todo!(),
        }
    }

    fn read_between(
        reader: &mut Peekable<Self>,
        start: Token,
        stop: Token,
    ) -> Result<Vec<Rc<dyn MalType>>, ParseError> {
        assert!(match (&start, &stop) {
            (Token::LeftParen, Token::RightParen) => true,
            (Token::LeftSquare, Token::RightSquare) => true,
            (Token::LeftCurly, Token::RightCurly) => true,
            _ => false,
        });
        assert_eq!(reader.next().unwrap().unwrap(), start);

        let mut list = Vec::new();
        loop {
            match reader.peek() {
                Some(Ok(token)) if *token == stop => break,
                Some(Ok(_)) => {
                    list.push(Reader::read_from(reader)?);
                }
                Some(Err(_)) => return Err(reader.next().unwrap().unwrap_err()),
                None => match start {
                    Token::LeftParen => return Err(ParseError::UnbalancedList),
                    Token::LeftSquare => return Err(ParseError::UnbalancedVec),
                    Token::LeftCurly => return Err(ParseError::UnbalancedMap),
                    _ => unreachable!(),
                },
            }
        }
        assert_eq!(reader.next().unwrap().unwrap(), stop);
        Ok(list)
    }

    fn read_atom(reader: &mut Peekable<Self>) -> ReaderResult {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new("^-?\\d+$").unwrap();
        }
        match reader.next() {
            Some(Ok(Token::Atom(atom))) => {
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
            Some(Ok(Token::String(string))) => Ok(Rc::from(MalString::from(string))),
            Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
            Some(Err(err)) => Err(err),
            None => panic!("Reached end of input."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{ParseError, Reader, Token};

    #[test]
    fn dont_read_whitespace_and_commas() {
        let result: Vec<_> = Reader::from(" \t\r\n,").collect();
        assert_eq!(result, vec![])
    }

    #[test]
    fn read_special_characters() {
        let result: Vec<_> = Reader::from("[]{}()'`~^@~@")
            .map(|token| token.unwrap())
            .collect();
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
                Token::TildeAt,
            ]
        )
    }

    #[test]
    fn read_strings() {
        let result: Vec<_> = Reader::from("\"one\" \"two\" \"three\"")
            .map(|token| token.unwrap())
            .collect();
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
        let result: Vec<_> = Reader::from(r#"backslash "\\" double-quote "\"" newline "\n" "#)
            .map(|token| token.unwrap())
            .collect();
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
    fn error_on_unbalanced_strings() {
        let mut reader = Reader::from("\"unbalanced\" \"strings");
        assert_eq!(reader.next(), Some(Ok(Token::String("unbalanced".into()))));
        assert_eq!(
            reader.next(),
            Some(Err(ParseError::UnbalancedString("strings".into())))
        );
    }

    #[test]
    fn read_symbols() {
        let result: Vec<_> = Reader::from("first second third")
            .map(|token| token.unwrap())
            .collect();
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
    fn dont_read_comments() {
        let result: Vec<_> = Reader::from("atom ; This is comment")
            .map(|token| token.unwrap())
            .collect();
        assert_eq!(result, vec![Token::Atom(String::from("atom")),])
    }
}
