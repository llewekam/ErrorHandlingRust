extern crate error_handling;

use std::env;
use error_handling::token;

fn main() {
    let token_string = command_line_token();

    token::do_token_stuff(&token_string);

    println!("{}", token_string);
}

fn command_line_token() -> String {
    if let Some(arg) = env::args().nth(1) {
        return arg;
    }

    "default".to_string()
}
