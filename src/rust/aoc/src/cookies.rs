use std::path::{Path, PathBuf};

use anyhow::anyhow;

use rusqlite::Connection;

fn firefox_profiles_dir() -> anyhow::Result<PathBuf> {
    #[cfg(target_os = "windows")]
    let profiles = ["Mozilla", "Firefox", "Profiles"];
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let profiles = ["", "Firefox", "Profiles"];

    let profiles = profiles.iter().collect::<PathBuf>();

    match dirs::config_dir() {
        Some(config) => Ok(config.join(profiles)),
        None => Err(anyhow!("could not get mozilla config path")),
    }
}

pub fn firefox_cookies_paths() -> anyhow::Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = firefox_profiles_dir()?
        .read_dir()?
        .filter_map(|p| {
            let Ok(p) = p else { return None };
            let cookie_file = p.path().join(PathBuf::from("cookies.sqlite"));
            cookie_file.exists().then_some(cookie_file)
        })
        .collect();

    if paths.is_empty() {
        Err(anyhow!("No cookie database paths found!"))
    } else {
        Ok(paths)
    }
}

pub fn token_from_cookies(cookie_file: &Path) -> anyhow::Result<String> {
    let conn = Connection::open(cookie_file)?;
    let mut stmt = conn.prepare(
        "SELECT value FROM moz_cookies WHERE name = 'session' AND host = '.adventofcode.com'",
    )?;

    match stmt.query_row([], |row| row.get::<usize, String>(0)) {
        Ok(token) => Ok(token),
        Err(_) => Err(anyhow!("No token found")),
    }
}

pub fn aoc_session_token_first() -> anyhow::Result<String> {
    let paths = firefox_cookies_paths()?;
    if paths.len() == 1 {
        token_from_cookies(&paths[0])
    } else {
        println!(
            "⚠️ Found multiple cookie databases, Select One: [0-{}]",
            paths.len() - 1
        );
        for (i, path) in paths.iter().enumerate() {
            println!("{}: {}", i, path.display());
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let index = input.trim().parse::<usize>()?;
        token_from_cookies(&paths[index])
    }
}
