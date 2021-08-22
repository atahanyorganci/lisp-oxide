use crate::types::{
    MalHashMap, MalInt, MalKeyword, MalList, MalString, MalSymbol, MalType, MalVec,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{convert::TryFrom, iter::Peekable, rc::Rc, str::FromStr};

#[derive(Debug)]
pub struct Reader {
    input: String,
    index: usize,
    error: Option<TokenizationError>,
}

impl From<String> for Reader {
    fn from(input: String) -> Self {
        Reader {
            input,
            index: 0,
            error: None,
        }
    }
}

impl FromStr for Reader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.into();
        Ok(Reader {
            input,
            index: 0,
            error: None,
        })
    }
}

impl Reader {
    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }

    pub fn read_from(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
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
                Err("unbalanced")
            }
            Some(Token::String(_)) | Some(Token::Atom(_)) => Reader::read_atom(reader),
            Some(Token::Caret) => Err("unimplemented"),
            Some(Token::Comment(_)) => unreachable!(),
            None => Err("end of input"),
        }
    }

    pub fn read_quote(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        let token = reader.next().unwrap();
        let symbol = match token {
            Token::TildeAt | Token::Apostrophe | Token::BackTick | Token::Tilde => {
                Reader::map_token_to_symbol(token).unwrap()
            }
            _ => panic!("Invalid token: {:?}", token),
        };
        let quoted = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, quoted])))
    }

    pub fn read_deref(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        assert_eq!(reader.next().unwrap(), Token::At);
        let symbol: Rc<dyn MalType> = Rc::from(MalSymbol::from("deref".to_string()));
        let derefed = Reader::read_from(reader)?;
        Ok(Rc::from(MalList::from(vec![symbol, derefed])))
    }

    fn map_token_to_symbol(token: Token) -> Result<Rc<dyn MalType>, ()> {
        let symbol = match token {
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
            | Token::Atom(_) => return Err(()),
        };
        Ok(Rc::from(MalSymbol::from(symbol.to_string())))
    }

    pub fn read_list(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        let list = Reader::read_between(reader, Token::LeftParen, Token::RightParen)?;
        Ok(Rc::from(MalList::from(list)))
    }

    pub fn read_vec(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        let list = Reader::read_between(reader, Token::LeftSquare, Token::RightSquare)?;
        Ok(Rc::from(MalVec::from(list)))
    }

    pub fn read_hashmap(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        let list = Reader::read_between(reader, Token::LeftCurly, Token::RightCurly)?;
        Ok(Rc::from(MalHashMap::try_from(list)?))
    }

    pub fn read_between(
        reader: &mut Peekable<Self>,
        start: Token,
        stop: Token,
    ) -> Result<Vec<Rc<dyn MalType>>, &'static str> {
        assert_eq!(reader.next().unwrap(), start);
        let mut list = Vec::new();
        loop {
            match reader.peek() {
                Some(token) if *token == stop => break,
                Some(_) => {
                    list.push(Reader::read_from(reader)?);
                }
                None => {
                    return Err("unbalanced");
                }
            }
        }
        assert_eq!(reader.next().unwrap(), stop);
        Ok(list)
    }

    pub fn read_atom(reader: &mut Peekable<Self>) -> Result<Rc<dyn MalType>, &'static str> {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new("^\\d*$").unwrap();
        }
        match reader.next() {
            Some(Token::Atom(atom)) => {
                if INT_RE.is_match_at(&atom, 0) {
                    let value = i64::from_str(&atom).unwrap();
                    Ok(Rc::from(MalInt::from(value)))
                } else if atom.starts_with(':') {
                    let word = &atom[1..];
                    Ok(Rc::from(MalKeyword::from(word.to_string())))
                } else {
                    Ok(Rc::from(MalSymbol::from(atom)))
                }
            }
            Some(Token::String(string)) => Ok(Rc::from(MalString::from(string))),
            _ => unimplemented!("Atoms and Strings are implemented"),
        }
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
    String(String),
    Comment(String),
    Atom(String),
}

pub fn is_special_char(ch: char) -> bool {
    match ch {
        '~' | '[' | ']' | '{' | '}' | '(' | ')' | '\'' | '`' | '"' | ',' | ';' => true,
        _ if ch.is_whitespace() => true,
        _ => false,
    }
}

impl Iterator for Reader {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let nth = self.input.chars().nth(self.index);

            match nth {
                Some(c) if c.is_whitespace() || c == ',' => {
                    self.index += 1;
                }
                Some('~') => {
                    let peeked = self.input.chars().nth(self.index + 1);
                    match peeked {
                        Some('@') => {
                            self.index += 2;
                            break Some(Token::TildeAt);
                        }
                        Some(_) => {
                            self.index += 1;
                            break Some(Token::Tilde);
                        }
                        None => {
                            self.index += 1;
                            break Some(Token::Tilde);
                        }
                    }
                }
                Some('[') => {
                    self.index += 1;
                    break Some(Token::LeftSquare);
                }
                Some(']') => {
                    self.index += 1;
                    break Some(Token::RightSquare);
                }
                Some('{') => {
                    self.index += 1;
                    break Some(Token::LeftCurly);
                }
                Some('}') => {
                    self.index += 1;
                    break Some(Token::RightCurly);
                }
                Some('(') => {
                    self.index += 1;
                    break Some(Token::LeftParen);
                }
                Some(')') => {
                    self.index += 1;
                    break Some(Token::RightParen);
                }
                Some('\'') => {
                    self.index += 1;
                    break Some(Token::Apostrophe);
                }
                Some('`') => {
                    self.index += 1;
                    break Some(Token::BackTick);
                }
                Some('^') => {
                    self.index += 1;
                    break Some(Token::Caret);
                }
                Some('@') => {
                    self.index += 1;
                    break Some(Token::At);
                }
                Some('"') => {
                    self.index += 1;

                    let mut remaining = match self.input.get(self.index..) {
                        Some(s) if s.is_empty() => {
                            self.error = Some(TokenizationError::UnbalancedString);
                            return None;
                        }
                        Some(s) => s.chars().peekable(),
                        None => unreachable!("Empty &str should have been returned!"),
                    };
                    let mut string = String::new();

                    while let Some(ch) = remaining.next() {
                        self.index += 1;
                        match ch {
                            '"' => {
                                return Some(Token::String(string));
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
                                None => break,
                            },
                            _ => string.push(ch),
                        }
                    }
                    self.error = Some(TokenizationError::UnbalancedString);
                    return None;
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
                    let _comment = Token::Comment(result);
                    break self.next();
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
                    break Some(Token::Atom(result));
                }
                None => break None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{Reader, Token};

    #[test]
    fn dont_tokenize_whitespace_and_commas() {
        let input = String::from(" \t\r\n,");
        let reader = Reader::from(input);
        let result: Vec<_> = reader.collect();
        assert_eq!(result, vec![])
    }

    #[test]
    fn tokenize_special_characters() {
        let input = String::from("[]{}()'`~^@~@");
        let reader = Reader::from(input);
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
    fn tokenize_strings() {
        let input = String::from("\"one\" \"two\" \"three\"");
        let reader = Reader::from(input);
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
    fn tokenize_strings_with_escape_sequences() {
        let input = String::from(r#"backslash "\\" double-quote "\"" newline "\n" "#);
        let reader = Reader::from(input);
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
    fn dont_tokenize_unbalanced_strings() {
        let input = String::from("\"unbalanced\" \"strings");
        let reader = Reader::from(input);
        let result: Vec<_> = reader.collect();
        assert_eq!(result, vec![Token::String(String::from("unbalanced")),]);
    }

    #[test]
    fn tokenize_symbols() {
        let input = String::from("first second third");
        let reader = Reader::from(input);
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
    fn tokenize_comments() {
        let input = String::from("atom ; This is comment");
        let reader = Reader::from(input);
        let result: Vec<_> = reader.collect();
        assert_eq!(
            result,
            vec![
                Token::Atom(String::from("atom")),
                Token::Comment(String::from("; This is comment")),
            ]
        )
    }
}