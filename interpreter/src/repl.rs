use super::{token, lexer};
use std::io::Write;

pub fn start(){
    let prompt: &'static str = ">> ";

    let reader = std::io::stdin();
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut read_buf = String::from("");
        if let Ok(_) = reader.read_line(&mut read_buf){
            let mut l = lexer::Lexer::new(&read_buf[..]);
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