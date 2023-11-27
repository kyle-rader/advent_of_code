use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::{aoc_client::AocClient, auth};

const CARGO_TOML: &str = "Cargo.toml";
const SRC: &str = "src";
const COMMON: &str = "common";
const SOLVERS: &str = "solvers";
const ALL_SOLVERS: &str = "solvers/*";

#[derive(Deserialize, Serialize, Debug)]
pub struct AocCargoWorkspace {
    pub workspace: Workspace,
}

impl AocCargoWorkspace {
    fn is_rust_aoc_workspace(&self) -> bool {
        let members: HashSet<&str> = self.workspace.members.iter().map(|s| s.as_str()).collect();
        members.contains(COMMON) && members.contains(ALL_SOLVERS)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
}

pub fn init() -> anyhow::Result<()> {
    todo!("Create workspace and common project")
}

pub fn new(year: usize, day: usize) -> Result<()> {
    let workspace = workspace_path()?;
    println!(
        "🛠️  Creating solver for {year} day {day} under {}",
        workspace.display()
    );

    // Does the solvers dir exist yet?
    let solver_dir = solver_dir(&workspace, year);
    if !solver_dir.exists() {
        std::fs::create_dir_all(&solver_dir)?;
    }

    // Does this year's project exist yet?
    if let Err(e) = ensure_solver_project(&solver_dir, year) {
        eprintln!(
            "Failed to ensure solver project exists at {}\n{e}",
            solver_dir.display()
        );

        eprintln!("Attempting to remove solver project dir");
        fs::remove_dir_all(&solver_dir)?;
        return Err(e);
    }

    // Does this day already exist? (Abort if so)
    if let Err(e) = ensure_day(&solver_dir, year, day) {
        eprintln!("Failed to generate day {day:02}.\n{e}");
        return Err(e);
    }

    println!("✅ Done!");
    Ok(())
}

fn ensure_day(solver_dir: &Path, year: usize, day: usize) -> anyhow::Result<()> {
    let day_id = format!("d{day:02}");
    let source = solver_dir.join(SRC).join(format!("{day_id}.rs"));
    let lib = solver_dir.join(SRC).join("lib.rs");

    if !source.is_file() {
        let token = auth::get_token()?;
        let client = AocClient::new(&token);
        let input = match client.input(year, day) {
            Ok(val) => format!("const INPUT: &str = \"{val}\";"),
            Err(e) => {
                eprintln!("Warning, could not get problem input. Using placeholder.\n{e}");
                String::from("const INPUT: &str = \"TBD\";")
            }
        };

        let content = format!(
            "#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {{
    Ok(0)
}}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {{
    Ok(0)
}}

#[cfg(test)]
mod tests_y{year} {{
    use super::*;

    #[test]
    fn part1_works() {{
        assert_eq!(part1(INPUT), Ok(42));
    }}

    #[test]
    fn part2_works() {{
        assert_eq!(part2(INPUT), Ok(42));
    }}
}}

#[cfg(test)]
{input}
"
        );
        fs::write(&source, content)?;

        // Append day to lib.rs
        let current_lib = fs::read_to_string(&lib)?;
        let mut mods: Vec<&str> = current_lib.lines().collect();
        let content = format!("mod {day_id};");
        mods.push(content.as_str());

        if let Err(e) = fs::write(&lib, mods.join("\n")) {
            return Err(anyhow!(format!(
                "Error appending mod {day_id}; to {}.\n{e}",
                lib.display()
            )));
        }
    } else {
        eprintln!("‼️ Source file for {day_id} already exists!");
    }
    Ok(())
}

fn ensure_solver_project(solver_dir: &Path, year: usize) -> anyhow::Result<()> {
    let cargo = solver_dir.join(CARGO_TOML);
    if cargo.is_file() {
        return Ok(());
    }

    // need to create this solver's project.
    // 1. Make Cargo.toml
    let toml = format!(
        "[package]
name = \"y{year}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
common = {{ path = \"../../common\" }}"
    );

    if let Err(e) = fs::write(cargo, toml) {
        eprintln!("{e}");
        return Err(anyhow!("Could not write solver cargo.toml"));
    }

    // 2. Make src/lib.rs
    let src = solver_dir.join("src");
    if !src.exists() {
        fs::create_dir_all(&src)?;
    }

    let lib = src.join("lib.rs");
    if !lib.exists() {
        fs::write(lib, "")?;
    }

    Ok(())
}

fn solver_dir(workspace: &Path, year: usize) -> PathBuf {
    workspace.join(SOLVERS).join(format!("y{year}"))
}

pub fn workspace_path() -> anyhow::Result<PathBuf> {
    std::env::current_dir()?
        .ancestors()
        .map(|dir| (dir, dir.join(CARGO_TOML)))
        .filter_map(|(dir, file)| {
            if !file.is_file() {
                return None;
            }
            let Ok(cargo_toml) = std::fs::read_to_string(&file) else {
                return None;
            };
            let Ok(cargo_toml): Result<AocCargoWorkspace, _> = toml::from_str(cargo_toml.as_str())
            else {
                return None;
            };
            cargo_toml.is_rust_aoc_workspace().then_some(dir.into())
        })
        .next()
        .ok_or_else(|| anyhow!("No workspace found"))
}
