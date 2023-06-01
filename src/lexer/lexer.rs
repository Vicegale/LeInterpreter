use crate::token::token::{self, Token, TokenKind}; 

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
       let mut l = Lexer {
        input: input,
        position: 0,
        read_position: 0,
        ch: String::from("\0"),
       };
       l.read_char();
       l
    }
    pub fn next_token(&mut self) -> token::Token {
        let tok: Token;
        match self.ch.as_str() {
            "=" => {
                tok = Lexer::new_token(TokenKind::ASSIGN, self.ch.clone());
            }
            "+" => {
                tok = Lexer::new_token(TokenKind::PLUS, self.ch.clone());
            }
            "(" => {
                tok = Lexer::new_token(TokenKind::LPAREN, self.ch.clone());
            }
            ")" => {
                tok = Lexer::new_token(TokenKind::RPAREN, self.ch.clone());
            }
            "{" => {
                tok = Lexer::new_token(TokenKind::LBRACE, self.ch.clone());
            }
            "}" => {
                tok = Lexer::new_token(TokenKind::RBRACE, self.ch.clone());
            }
            "," => {
                tok = Lexer::new_token(TokenKind::COMMA, self.ch.clone());
            }
            ";" => {
                tok = Lexer::new_token(TokenKind::SEMICOLON, self.ch.clone());
            }
            "\0" => {
                tok = Lexer::new_token(TokenKind::EOF, self.ch.clone());
            }
            _ => {
                tok = Lexer::new_token(TokenKind::ILLEGAL, self.ch.clone());
            }
        }
        self.read_char();
        tok
    }
    fn new_token(kind: token::TokenKind, ch: String) -> Token {
        Token {
            kind: kind,
            literal: ch
        }
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = String::from("\0");
        }
        else {
            self.ch = self.input.chars().nth(self.read_position).unwrap().into();
        }
        self.position = self.read_position;
        self.read_position += 1;
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
}
