use crate::aoc_client::AocClient;
use crate::cookies::aoc_session_token_first;

pub fn login(token: Option<String>) -> anyhow::Result<()> {
    // Get
    let token = match token {
        Some(token) => {
            println!("Using token provided on CLI");
            token
        }
        None => {
            println!("Using token from FireFox cookies");
            aoc_session_token_first()?
        }
    };

    // Test
    let client = AocClient::new(token);
    let user_name = client.user_name()?;
    println!("Welcome, {user_name}!");

    // Save
    // todo - save it
    Ok(())
}

// todo: function to retrieve cached token
// todo: logout
