use std::fs::File;
use std::path::PathBuf;

use crate::aoc_client::AocClient;
use crate::cookies::aoc_session_token_first;
use anyhow::anyhow;
use directories::ProjectDirs;

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
    Ok(())
}

fn save_token(token: String) -> anyhow::Result<()> {
    let cache_file = cache_file()?;
    File::Ok(())
}

fn cache_file() -> anyhow::Result<PathBuf> {
    Ok(ProjectDirs::from("com", "advent_of_code", "aoc_cli")
        .ok_or_else(|| anyhow!("Unable to get cache dir!"))?
        .cache_dir()
        .join("aoc.cache"))
}

// todo: function to retrieve cached token
// todo: logout
