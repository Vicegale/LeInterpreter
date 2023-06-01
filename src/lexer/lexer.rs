use crate::token::token::{self, Token, TokenKind}; 

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
       let mut l = Lexer {
        input: input,
        position: 0,
        read_position: 0,
        ch: '\0',
       };
       l.read_char();
       l
    }
    pub fn next_token(&mut self) -> token::Token {
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                tok = Lexer::new_token(TokenKind::ASSIGN, self.ch.into());
            }
            '+' => {
                tok = Lexer::new_token(TokenKind::PLUS, self.ch.into());
            }
            '(' => {
                tok = Lexer::new_token(TokenKind::LPAREN, self.ch.into());
            }
            ')' => {
                tok = Lexer::new_token(TokenKind::RPAREN, self.ch.into());
            }
            '{' => {
                tok = Lexer::new_token(TokenKind::LBRACE, self.ch.into());
            }
            '}' => {
                tok = Lexer::new_token(TokenKind::RBRACE, self.ch.into());
            }
            ',' => {
                tok = Lexer::new_token(TokenKind::COMMA, self.ch.into());
            }
            ';' => {
                tok = Lexer::new_token(TokenKind::SEMICOLON, self.ch.into());
            }
            '\0' => {
                tok = Lexer::new_token(TokenKind::EOF, self.ch.into());
            }
            _ => {
                if Lexer::is_identifier_char(self.ch) {
                    let literal = self.read_identifier();
                    let kind = token::resolve_identifier(&literal);
                    tok = Lexer::new_token(kind, literal);
                    return tok;
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    let kind = TokenKind::INT;
                    tok = Lexer::new_token(kind, literal);
                    return tok;
                }
                else {
                    tok = Lexer::new_token(TokenKind::ILLEGAL, self.ch.into());
                }
            }
        }
        self.read_char();
        tok
    }
    fn is_identifier_char(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn new_token(kind: token::TokenKind, ch: String) -> Token {
        Token {
            kind: kind,
            literal: ch
        }
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars().nth(self.read_position).unwrap().into();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input.chars().skip(position).take(self.read_position-position-1).collect()

    }
    
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_identifier_char(self.ch) {
            self.read_char();
        }
        self.input.chars().skip(position).take(self.read_position-position-1).collect()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\r' || self.ch == '\t' {
            self.read_char();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        struct ExpectedToken {
            expected_kind: TokenKind,
            expected_literal: String
        }
        let expected_output = [ 
            ExpectedToken {expected_kind: TokenKind::ASSIGN, expected_literal: String::from("=")},
            ExpectedToken {expected_kind: TokenKind::PLUS, expected_literal: String::from("+")},
            ExpectedToken {expected_kind: TokenKind::LPAREN, expected_literal: String::from("(")},
            ExpectedToken {expected_kind: TokenKind::RPAREN, expected_literal: String::from(")")},
            ExpectedToken {expected_kind: TokenKind::LBRACE, expected_literal: String::from("{")},
            ExpectedToken {expected_kind: TokenKind::RBRACE, expected_literal: String::from("}")},
            ExpectedToken {expected_kind: TokenKind::COMMA, expected_literal: String::from(",")},
            ExpectedToken {expected_kind: TokenKind::SEMICOLON, expected_literal: String::from(";")},
            ExpectedToken {expected_kind: TokenKind::EOF, expected_literal: String::from("\0")}
            ];
        let mut l = Lexer::new(input);
        for out in expected_output.iter() {
            let t = l.next_token();
            assert_eq!(t.kind, out.expected_kind);
            assert_eq!(t.literal, out.expected_literal);
        }
    }

    #[test]
    fn test_basic_lexics() {
        let input = String::from("let five = 5;
                                 let ten = 10;

                                 let add = fn(x, y) {
                                    x + y;
                                 }

                                 let result = add(five, ten);
                                 ");
        struct ExpectedToken {
            expected_kind: TokenKind,
            expected_literal: String
        }
        let expected_output = [
            ExpectedToken {expected_kind: TokenKind::LET, expected_literal: String::from("let")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("five")},
            ExpectedToken {expected_kind: TokenKind::ASSIGN, expected_literal: String::from("=")},
            ExpectedToken {expected_kind: TokenKind::INT, expected_literal: String::from("5")},
            ExpectedToken {expected_kind: TokenKind::SEMICOLON, expected_literal: String::from(";")},
            ExpectedToken {expected_kind: TokenKind::LET, expected_literal: String::from("let")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("ten")},
            ExpectedToken {expected_kind: TokenKind::ASSIGN, expected_literal: String::from("=")},
            ExpectedToken {expected_kind: TokenKind::INT, expected_literal: String::from("10")},
            ExpectedToken {expected_kind: TokenKind::SEMICOLON, expected_literal: String::from(";")},
            ExpectedToken {expected_kind: TokenKind::LET, expected_literal: String::from("let")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("add")},
            ExpectedToken {expected_kind: TokenKind::ASSIGN, expected_literal: String::from("=")},
            ExpectedToken {expected_kind: TokenKind::FUNCTION, expected_literal: String::from("fn")},
            ExpectedToken {expected_kind: TokenKind::LPAREN, expected_literal: String::from("(")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("x")},
            ExpectedToken {expected_kind: TokenKind::COMMA, expected_literal: String::from(",")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("y")},
            ExpectedToken {expected_kind: TokenKind::RPAREN, expected_literal: String::from(")")},
            ExpectedToken {expected_kind: TokenKind::LBRACE, expected_literal: String::from("{")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("x")},
            ExpectedToken {expected_kind: TokenKind::PLUS, expected_literal: String::from("+")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("y")},
            ExpectedToken {expected_kind: TokenKind::SEMICOLON, expected_literal: String::from(";")},
            ExpectedToken {expected_kind: TokenKind::RBRACE, expected_literal: String::from("}")},
            ExpectedToken {expected_kind: TokenKind::LET, expected_literal: String::from("let")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("result")},
            ExpectedToken {expected_kind: TokenKind::ASSIGN, expected_literal: String::from("=")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("add")},
            ExpectedToken {expected_kind: TokenKind::LPAREN, expected_literal: String::from("(")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("five")},
            ExpectedToken {expected_kind: TokenKind::COMMA, expected_literal: String::from(",")},
            ExpectedToken {expected_kind: TokenKind::IDENT, expected_literal: String::from("ten")},
            ExpectedToken {expected_kind: TokenKind::RPAREN, expected_literal: String::from(")")},
            ExpectedToken {expected_kind: TokenKind::SEMICOLON, expected_literal: String::from(";")},
            ExpectedToken {expected_kind: TokenKind::EOF, expected_literal: String::from("\0")}
        ];

        let mut l = Lexer::new(input);
        let mut test_number = 0;
        for out in expected_output.iter() {
            test_number += 1;
            let t = l.next_token(); 
            println!("{}", t.literal);
            assert_eq!(t.kind, out.expected_kind, "Failed test #{}. Found literal {}", test_number, t.literal);
            assert_eq!(t.literal, out.expected_literal, "Failed test #{}. Found literal {}", test_number, t.literal);
        }
    }
}
