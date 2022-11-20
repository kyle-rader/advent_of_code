use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;

pub fn login(token: Option<String>) -> anyhow::Result<()> {
    // Get
    let token = token.unwrap_or(token_from_cookies()?);
    println!("Using token: {token}");
    // Test
    // Save
    Ok(())
}

fn token_from_cookies() -> anyhow::Result<String> {
    Ok(String::from("foobar"))
}

fn mozilla_profiles() -> anyhow::Result<Vec<PathBuf>> {
    if let Some(mut config) = dirs::config_dir() {
        config.push("Mozilla");
        config.push("Firefox");
        config.push("Profiles");
    } else {
        Err(anyhow!("could not get mozilla config path"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn mozilla_dir_exists() {}
}
