use std::path::{Path, PathBuf};

use anyhow::anyhow;

use rusqlite::Connection;

fn firefox_profiles_dir() -> anyhow::Result<PathBuf> {
    #[cfg(target_os = "windows")]
    let profiles = ["Mozilla", "Firefox", "Profiles"];
    #[cfg(target_os = "macos")]
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

pub fn aoc_session_token(cookie_file: &Path) -> anyhow::Result<String> {
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
    aoc_session_token(&firefox_cookies_paths()?[0])
}
