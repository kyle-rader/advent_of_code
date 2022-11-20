use dirs;
use std::path::{Path, PathBuf};

use anyhow::anyhow;

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

fn firefox_cookies_path() -> anyhow::Result<PathBuf> {
    let profiles = firefox_profiles_dir()?;
    let default = profiles
        .read_dir()?
        .filter_map(|p| {
            if let Ok(p) = p {
                println!("checking dir entry: {}", p.path().display());
                let cookie_file = p.path().join(PathBuf::from("cookies.sqlite"));
                if cookie_file.exists() {
                    println!("This path!");
                    Some(cookie_file)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .nth(0);

    default.ok_or(anyhow!("no firefox cookies found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn mozilla_dir_exists() {
        match firefox_cookies_path() {
            Ok(profile) => assert!(profile.exists()),
            Err(e) => panic!("{:?}", e),
        }
    }
}
