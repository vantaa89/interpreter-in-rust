mod token;
mod lexer;
mod repl;
use users::{get_user_by_uid, get_current_uid};

fn main() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hello, {}! This is the Monkey programming Language!", user.name().to_str().unwrap());
    println!("Feel free to type in commands");
    repl::start();
}
