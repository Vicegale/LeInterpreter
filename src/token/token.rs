pub struct Token  {
     pub kind: TokenKind,
     pub literal: String 
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenKind {
    ILLEGAL,
    EOF,
    
    IDENT,
    INT,
    
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NEQ,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    SEMICOLON,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}
