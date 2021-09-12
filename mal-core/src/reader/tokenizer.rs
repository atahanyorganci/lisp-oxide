use super::{token::is_special_char, FullToken, ParseError, Token};

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(input: &'a str) -> Self {
        Tokenizer { input, index: 0 }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<FullToken, ParseError>;

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
                                return Some(Err(ParseError::UnbalancedEmptyString));
                            } else {
                                return Some(Err(ParseError::UnbalancedString(string)));
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
                            None => return Some(Err(ParseError::UnbalancedString(string))),
                        },
                        _ => string.push(ch),
                    }
                };
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
        Some(Ok(FullToken::new(token, start, self.index)))
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{ParseError, Token};

    use super::Tokenizer;

    #[test]
    fn tokenize_whitspace_and_commans() {
        let result: Vec<_> = Tokenizer::from(" \t\r\n")
            .map(|token| token.unwrap())
            .collect();
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
    fn tokenize_incomplete_strings() {
        let mut tokenizer = Tokenizer::from("\"strings");
        assert_eq!(
            tokenizer.next(),
            Some(Err(ParseError::UnbalancedString("strings".into())))
        );
    }

    #[test]
    fn tokenize_incomplete_empty_strings() {
        let mut tokenizer = Tokenizer::from("\"");
        assert_eq!(
            tokenizer.next(),
            Some(Err(ParseError::UnbalancedEmptyString))
        );
    }

    #[test]
    fn tokenize_comments_strings() {
        let result: Vec<_> = Tokenizer::from("bruh ; This is a comment")
            .map(|token| token.unwrap())
            .collect();
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
