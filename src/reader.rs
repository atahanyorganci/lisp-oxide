use std::str::FromStr;

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
                    break Some(Token::Comment(result));
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
