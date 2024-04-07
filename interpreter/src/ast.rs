use super::token;
use std::rc::Rc;
use std::any::Any;

// All nodes in AST needs to implement Node trait
pub trait Node {
    // returns the literal value that corresponds to the token
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;   // required for downcasting from dyn Statement to concrete struct types
}

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String{
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

pub struct LetStatement {               // e.g. let x = 5;
    pub token: token::Token,
    pub name: Rc<Identifier>,           // Identifier, e.g. x
    pub value: Rc<dyn Expression>       // The expression that generates the value, e.g. 5
}

impl Node for LetStatement{
    fn token_literal(&self) -> String {
        format!("{}", self.token)
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}  
    fn as_any(&self) -> &dyn Any {
        self
    }  
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        format!("{}", self.token)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    
}
