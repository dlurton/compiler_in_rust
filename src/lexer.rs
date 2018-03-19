
use std::str::Chars;

use input::CharsReader;
use input::Span;
use input::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LiteralInt32(i32),
    Identifier(String)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

fn is_white(chr: char) -> bool {
    match chr {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false
    }
}

fn is_digit(chr: char) -> bool {
    chr >= '0' && chr <= '9'
}

fn is_letter(chr: char) -> bool {
    (chr >= 'a' && chr <= 'z') || (chr >= 'A' && chr <= 'Z')
}

pub struct Lexer<'a> {
    reader: CharsReader<'a>
}

impl <'a> Lexer<'a> {
    pub fn new(chars: Chars) -> Lexer {
        return Lexer { reader: CharsReader::new(chars) }
    }

    pub fn next(&mut self) -> Option<Token> {
        if !self.reader.has_more() {
            None
        } else {
            self.eat_white();
            if let Some(token) = self.read_literal_number() {
                Some(token)
            } else if let Some(token) = self.read_identifier() {
                Some(token)
            } else {
                panic!("Invalid character: {:?}!", self.reader.peek())
            }
        }
    }

    fn eat_white(&mut self) {
        while let Some(c) = self.reader.peek() {
            if !is_white(c) {
                break;
            }
            self.reader.next();
        }
    }

    fn read_literal_number(&mut self) -> Option<Token> {
        match self.reader.peek() {
            None => None,
            Some(c) => {
                if !is_digit(c) {
                    None
                } else {
                    let mut buf = String::new();
                    buf.push(c);
                    self.reader.next();
                    let start = self.reader.loc();
                    while let Some(c) = self.reader.peek() {
                        if !is_digit(c) {
                            break;
                        }
                        buf.push(c);

                        self.reader.next();
                    }

                    // For now we don't care too much about error handling.
                    let maybe_int = buf.parse::<i32>();
                    match maybe_int {
                        Ok(i) => Some(Token::new(TokenKind::LiteralInt32(i), Span::new(start, self.reader.loc()))),
                        Err(_e) => panic!("Crap, it wasn't an integer!")
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Option<Token> {
        match self.reader.peek() {
            None => None,
            Some(c) => {
                if !is_letter(c) {
                    None
                } else {
                    let mut buf = String::new();
                    buf.push(c);
                    self.reader.next();
                    let start = self.reader.loc();
                    while let Some(c) = self.reader.peek() {
                        if !is_letter(c) && !is_digit(c) {
                            break;
                        }
                        buf.push(c);

                        self.reader.next();
                    }
                    Some(Token::new(TokenKind::Identifier(buf), Span::new(start, self.reader.loc())))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lexer_test() {
        let mut l = Lexer::new("  123  \n 456 \nabc\na123 ".chars());

        assert_eq!(Token::new(TokenKind::LiteralInt32(123), Span::new(Location::new(1, 3), Location::new(1, 5))), l.next().unwrap());
        assert_eq!(Token::new(TokenKind::LiteralInt32(456), Span::new(Location::new(2, 2), Location::new(2, 4))), l.next().unwrap());
        assert_eq!(Token::new(TokenKind::Identifier(String::from("abc")), Span::new(Location::new(3, 1), Location::new(3, 3))), l.next().unwrap());
        assert_eq!(Token::new(TokenKind::Identifier(String::from("a123")), Span::new(Location::new(4, 1), Location::new(4, 4))), l.next().unwrap());
    }
}


  
