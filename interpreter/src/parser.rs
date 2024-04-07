use super::{token, lexer, ast};
use ast::*;
use std::rc::Rc;

pub struct Parser<'a> {
    l: &'a mut lexer::Lexer<'a>,
    pub cur_token: Option<token::Token>,
    pub peek_token: Option<token::Token>
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut lexer::Lexer<'a>) -> Parser<'a> {
        let mut p = Parser{
            l: lexer, 
            cur_token: None, 
            peek_token: None
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
        while self.cur_token.clone().unwrap() != token::Token::Eof {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            }
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.clone().unwrap() {
            token::Token::Let => {
                let parsed_let = self.parse_let_statement();
                parsed_let.map(|boxed| Box::new(*boxed) as Box<dyn ast::Statement>)
            }
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::LetStatement>> {
        let token = self.cur_token.clone().unwrap();
        let name: Rc<ast::Identifier>;
        let value: Rc<dyn ast::Expression> = Rc::new(ast::Identifier{token: token::Token::Eof, value: "".to_string()}); // value is a placeholder
        if let token::Token::Ident(ident_name) = self.peek_token.clone().unwrap() {
            name = Rc::new(ast::Identifier{token: token::Token::Ident(ident_name.clone()), value: ident_name.clone()});
            self.next_token();  // consume ident
        } else {
            return None;
        }

        if self.peek_token.clone().unwrap() != token::Token::Assign {
            return None;
        }
        self.next_token();      // consume =

        while self.cur_token.clone().unwrap() != token::Token::Semicolon {
            self.next_token();
            // TODO
        }
        self.next_token();      // consume semicolon
        return Some(Box::new(ast::LetStatement{token, name, value}));
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
        if program.statements.len() != 3{
            panic!("program.statements does not contain 3 statements, got={}", program.statements.len())
        }
        let expected_identifiers = ["x", "y", "foobar"];
        for i in 0..3 {
            let ident = expected_identifiers[i].to_string();
            test_let_statement(program.statements[i].as_ref() as &dyn ast::Statement, ident);   
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
}