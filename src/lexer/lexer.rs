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
        match self.ch {
            '=' => {
                tok = Lexer::new_token(TokenKind::ASSIGN, self.ch);
            }
            '+' => {
                tok = Lexer::new_token(TokenKind::PLUS, self.ch);
            }
            '(' => {
                tok = Lexer::new_token(TokenKind::LPAREN, self.ch);
            }
            ')' => {
                tok = Lexer::new_token(TokenKind::RPAREN, self.ch);
            }
            '{' => {
                tok = Lexer::new_token(TokenKind::LBRACE, self.ch);
            }
            '}' => {
                tok = Lexer::new_token(TokenKind::RBRACE, self.ch);
            }
            ',' => {
                tok = Lexer::new_token(TokenKind::COMMA, self.ch);
            }
            ';' => {
                tok = Lexer::new_token(TokenKind::SEMICOLON, self.ch);
            }
            _ => {
                tok = Lexer::new_token(TokenKind::ILLEGAL, self.ch);
            }
        }
        self.read_char();
        tok
    }
    fn new_token(kind: token::TokenKind, ch: char) -> Token {
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
            self.ch = self.input.chars().nth(self.read_position).unwrap();
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
        let mut l = Lexer::new(input);
        let t = l.next_token();
        assert_eq!(t.kind, TokenKind::ASSIGN);
        assert_eq!(t.literal, '=');
        assert_eq!(l.next_token().kind, TokenKind::PLUS);
        assert_eq!(l.next_token().kind, TokenKind::LPAREN);
        assert_eq!(l.next_token().kind, TokenKind::RPAREN);
        assert_eq!(l.next_token().kind, TokenKind::LBRACE);
        assert_eq!(l.next_token().kind, TokenKind::RBRACE);
        assert_eq!(l.next_token().kind, TokenKind::COMMA);
        assert_eq!(l.next_token().kind, TokenKind::SEMICOLON);
    }
}
