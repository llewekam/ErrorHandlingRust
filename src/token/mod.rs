
pub fn do_token_stuff(token: &str) -> Result<(), String> {
    if token == "" {
        panic!("Token is empty");
    }

    println!("Got token string: {}", token);

    if token == "invalid" {
        return Result::Err("The token was invalid".to_string())
    }

    Ok(())
}
