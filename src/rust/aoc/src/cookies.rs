use std::path::{Path, PathBuf};

use anyhow::anyhow;

use rusqlite::Connection;

fn firefox_profiles_dir() -> anyhow::Result<PathBuf> {
    match dirs::config_dir() {
        Some(config) => Ok(config.join(
            ["Mozilla", "Firefox", "Profiles"]
                .iter()
                .collect::<PathBuf>(),
        )),
        None => Err(anyhow!("could not get mozilla config path")),
    }
}

pub fn firefox_cookies_paths() -> anyhow::Result<Vec<PathBuf>> {
    Ok(firefox_profiles_dir()?
        .read_dir()?
        .filter_map(|p| {
            let Ok(p) = p else { return None };
            let cookie_file = p.path().join(PathBuf::from("cookies.sqlite"));
            cookie_file.exists().then_some(cookie_file)
        })
        .collect())
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
