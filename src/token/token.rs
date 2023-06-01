use phf::phf_map;

pub struct Token  {
     pub kind: TokenKind,
     pub literal: String 
}

pub struct Keyword<'a> {
    pub string: &'a str,
    pub keyword: TokenKind
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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

pub const KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "let" => TokenKind::LET,
    "fn" => TokenKind::FUNCTION
};

pub fn resolve_identifier(str: &str) -> TokenKind {
    let keyword = KEYWORDS.get(str);
    match keyword {
        Some(x) => return x.clone(),
        None => return TokenKind::IDENT
    }
}

#[test]
fn test_resolve_identifier() {
    assert_eq!(TokenKind::LET, resolve_identifier("let"));
    assert_eq!(TokenKind::IDENT, resolve_identifier("a"));
}
