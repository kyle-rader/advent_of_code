use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use crate::aoc_client::AocClient;
use crate::cookies::aoc_session_token_first;
use anyhow::anyhow;
use directories::ProjectDirs;

pub fn login(token: Option<String>) -> anyhow::Result<()> {
    // Get
    let token = match token {
        Some(token) => {
            println!("📝Using token provided on CLI");
            token
        }
        None => {
            println!("🍪 Using token from FireFox cookies");
            aoc_session_token_first()?
        }
    };

    // Test
    let client = AocClient::new(&token);
    let user_name = client.user_name()?;
    println!("✅ Token works!");

    // Save
    let cache_file = save_token(&token)?;
    println!("💾 Token saved at {}", &cache_file.display());

    println!("🚀 Welcome, {user_name}! Happy solving 🎉");
    Ok(())
}

fn save_token(token: &String) -> anyhow::Result<PathBuf> {
    let cache_file = cache_file()?;
    let mut file = File::create(&cache_file)?;
    file.write_all(token.as_bytes())?;
    Ok(cache_file)
}

fn cache_file() -> anyhow::Result<PathBuf> {
    let Some(project_dir) = ProjectDirs::from("com", "advent_of_code", "aoc_cli") else { return Err(anyhow!("Could not get project directory")) };
    let cache_dir = project_dir.cache_dir();

    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir)?;
    }

    Ok(cache_dir.join("aoc.cache"))
}

// todo: function to retrieve cached token
// todo: logout
