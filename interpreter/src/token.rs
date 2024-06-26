use std::fmt;
use std::collections::HashMap;
use enum_as_inner::EnumAsInner;

#[derive(Debug, PartialEq, Clone, EnumAsInner)]
pub enum Token {
    Illegal(char),
    Eof,
    Ident(String),
    Int(String),
    Assign,         // =
    Plus,           // +
    Minus,          // -
    Bang,           // !
    Asterisk,       // *
    Slash,          // /
    Lt,             // <
    Gt,             // >
    Comma,          // ,
    Semicolon,      // ;
    Lparen,         // (
    Rparen,         // )
    Lbrace,         // {
    Rbrace,         // }
    Function,       // fn
    Let,            // let
    True,           // true,
    False,          // false
    If,             // if
    Else,           // else
    Return,         // return
}

impl fmt::Display for Token{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "EOF"),
            Token::Ident(t) => write!(f, "{}", t),
            Token::Int(t) => write!(f, "{}", t),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace  => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::Illegal(t) => panic!("Illegal Token: {:?}", t),
        }
    }
}

pub fn lookup_ident(ident: &String) -> Token {
    let keywords: HashMap<String, Token> = HashMap::from([
        (String::from("fn"), Token::Function),
        (String::from("let"), Token::Let),
        (String::from("true"), Token::True),
        (String::from("false"), Token::False),
        (String::from("if"), Token::If),
        (String::from("else"), Token::Else),
        (String::from("return"), Token::Return)
    ]);
    match keywords.get(ident.as_str()) {
        Some(t) => (*t).clone(),
        None => Token::Ident(ident.clone())
    }
}