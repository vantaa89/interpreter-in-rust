use super::{token, lexer};
use std::io::Write;

pub fn start(){
    let prompt: &'static str = ">> ";

    let mut reader = std::io::stdin();
    let mut read_buf = String::from("");
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        if let Ok(_) = reader.read_line(&mut read_buf){
            let mut l = lexer::Lexer::new(read_buf.clone());
            loop {
                let token = l.next_token();
                println!("{}", token);
                if token == token::Token::Eof {
                    break;
                }   
            }
        }

    }
}