use super::{token, lexer, ast};
use ast::*;
use std::rc::Rc;

pub struct Parser<'a> {
    l: &'a mut lexer::Lexer<'a>,
    pub cur_token: Option<token::Token>,
    pub peek_token: Option<token::Token>, 
    errors: Vec<String>
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let mut p = Parser{
            l: lexer, 
            cur_token: None, 
            peek_token: None,
            errors: Vec::<String>::new()
        };
        // set cur_token and peek_token by reading twice
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.l.next_token());
    }

    fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program = ast::Program{statements: vec![]};
        while *self.cur_token.as_ref().unwrap() != token::Token::Eof {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            } else {
                return None;    // Parsing failed    
            }
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.as_ref().unwrap() {
            token::Token::Let => {
                let parsed_let = self.parse_let_statement();
                parsed_let.map(|boxed| Box::new(*boxed) as Box<dyn ast::Statement>)
            },
            token::Token::Return => {
                let parsed_return = self.parse_return_statement();
                parsed_return.map(|boxed| Box::new(*boxed) as Box<dyn ast::Statement>)
            }
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::LetStatement>> {
        let token = self.cur_token.clone().unwrap();
        let name: Rc<ast::Identifier>;
        let value: Rc<dyn ast::Expression> = Rc::new(ast::Identifier{token: token::Token::Eof, value: "".to_string()}); // placeholder
        
        let ident_token = token::Token::Ident("".to_string());

        if self.expect_peek(ident_token) {      // expect an identifier token to appear
            if let token::Token::Ident(ident_name) =  self.cur_token.as_ref().unwrap() {
                name = Rc::new(ast::Identifier{token: token::Token::Ident(ident_name.clone()), value: ident_name.clone()});
            } else {
                panic!();
            }
        } else {
            return None;
        }

        if !self.expect_peek(token::Token::Assign){
            return None;
        }

        while *self.cur_token.as_ref().unwrap() != token::Token::Semicolon {
            self.next_token();
            // TODO
        }
        self.next_token();      // consume semicolon
        return Some(Box::new(ast::LetStatement{token, name, value}));
    }

    fn parse_return_statement(&mut self) -> Option<Box<ast::ReturnStatement>> {
        let token = self.cur_token.clone().unwrap();
        let return_value: Rc<dyn ast::Expression> = Rc::new(ast::Identifier{token: token::Token::Eof, value: "".to_string()}); // placeholder

        while *self.cur_token.as_ref().unwrap() != token::Token::Semicolon {
            self.next_token();
            // TODO
        }
        self.next_token();      // consume semicolon
        return Some(Box::new(ast::ReturnStatement{token, return_value}));
    }

    fn expect_peek(&mut self, t: token::Token) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(&t);
            println!("{}", self.errors[self.errors.len()-1]);
            return false;
        }
    }

    // checks if the type of token matches
    // the literals are unimportant for Token::Int or Token::Ident
    fn peek_token_is(&mut self, t: &token::Token) -> bool {
        let peek_token = self.peek_token.as_ref().unwrap();
        if *peek_token == *t {
            return true;
        }
        if peek_token.is_ident() && t.is_ident() || peek_token.is_int() && t.is_int() {
            return true;
        } 
        return false;
    }

    fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: &token::Token){
        let msg = format!("expected new token to be {}, got {} instead", *t, self.peek_token.as_ref().unwrap());
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod token_test{
    use super::*;
    #[test]
    fn test_let_statements(){
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 8383838;
        ";
        let mut lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = match parser.parse_program() {
            Some(p) => p,
            None => panic!("parse_program() returned None")
        };
        check_parser_error(&parser);
        if program.statements.len() != 3{
            panic!("program.statements does not contain 3 statements, got={}", program.statements.len())
        }
        let expected_identifiers = ["x", "y", "foobar"];
        for i in 0..3 {
            let ident = expected_identifiers[i].to_string();
            if !test_let_statement(program.statements[i].as_ref() as &dyn ast::Statement, ident){
                return;
            }
        }

    }

    fn test_let_statement(s: &dyn ast::Statement, name: String) -> bool {
        if s.token_literal() != "let".to_string() {
            println!("s.token_literal() not 'let'. got={}", s.token_literal());
            return false;
        }
        if let Some(let_statement) = s.as_any().downcast_ref::<ast::LetStatement>() {
            if let_statement.name.value != name {
                println!("let_statement.name.value not {}. got={}", name, let_statement.name.value);
                return false;
            }
            if let_statement.name.as_ref().token_literal() != name {
                println!("let_statement.name.token_literal() not {}. got={}",
                name, let_statement.name.as_ref().token_literal());
            }
        } else {
            println!("s not ast::LetStatement.");
            return false;
        }
        true
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
        return 10;
        return 993322;";

        let mut lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);
        let program = match parser.parse_program() {
            Some(p) => p,
            None => panic!("parse_program() returned None")
        };
        check_parser_error(&parser);
        if program.statements.len() != 3{
            panic!("program.statements does not contain 3 statements, got={}", program.statements.len())
        }
        for i in 0..3 {
            let return_statement = program.statements[i].as_any().downcast_ref::<ast::ReturnStatement>();
            if let Some(statement) = return_statement {
                if statement.token_literal() != "return" {
                    println!("statement.token_literal() not 'return', got {}.", statement.token_literal());
                }
            } else {
                println!("statement not ast::ReturnStatement.");
            }
        }
    }

    fn check_parser_error(p: &Parser){
        let errors = p.errors();
        if errors.len() == 0{
            return;
        }
        println!("parser has {} errors", errors.len());
        for msg in &errors{
            println!("parser error: {}", msg);
        }
        panic!();
    }
}