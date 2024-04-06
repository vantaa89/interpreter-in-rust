use super::token;
use super::token::Token;

pub struct Lexer<'a>{
    input: &'a str,      
    position: usize,       // the current position in the input
    read_position: usize,  // the currently read position in the input
    ch: char,              // the character that is processed
}

impl<'a> Lexer<'a>{
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer {input, position: 0, read_position: 0, ch: '\0'};
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => Token::Bang,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            t => {
                if Self::is_letter(t) {
                    return self.read_identifier();
                } else if Self::is_digit(t) {
                    return self.read_digit();
                } else {
                    Token::Illegal(t)
                }
            }
        };
        self.read_char();
        return tok;
    }

    fn read_identifier(&mut self) -> Token {
        // reads the input unitl it meets a non-letter character
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }    
        let token_string = String::from(&self.input[position..self.position]);
        token::lookup_ident(&token_string)     // returns the token for the identifier or a reserved keyword
    }

    fn read_digit(&mut self) -> Token {
        // reads the input unitl it meets a non-letter character
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }    
        Token::Int(String::from(&self.input[position..self.position]))
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

}

#[cfg(test)]
mod lexer_test{
    use super::*;
    #[test]
    fn test_next_token(){
        let input: &str = "let five = 5;
        let ten = 10;
        let add = fn(x, y){
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if(5 < 10) {
            return true;
        } else {
            return false;
        }
        ";

        let expected_output = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,             
            Token::Ident(String::from("ten")),             
            Token::Assign,             
            Token::Int(String::from("10")),             
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")), 
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,             
            Token::Let,             
            Token::Ident(String::from("result")),             
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::Lt,
            Token::Int(String::from("10")),
            Token::Gt,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(String::from("5")),
            Token::Lt,
            Token::Int(String::from("10")),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Eof
            ];

        let mut lexer = Lexer::new(input);
        
        for i in 0..expected_output.len() {
            let tok = lexer.next_token();
            assert_eq!(tok, expected_output[i], "Token number {i}: expected {}, got {}", expected_output[i], tok);
        }
    }
}