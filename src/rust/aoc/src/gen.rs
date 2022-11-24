use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::{aoc_client::AocClient, auth};

const COMMON: &str = "common";
const SOLVERS: &str = "solvers/*";

#[derive(Deserialize, Serialize, Debug)]
pub struct AocCargoWorkspace {
    pub workspace: Workspace,
}

impl AocCargoWorkspace {
    fn is_rust_aoc_workspace(&self) -> bool {
        let members: HashSet<&str> = self.workspace.members.iter().map(|s| s.as_str()).collect();
        members.contains(COMMON) && members.contains(SOLVERS)
    }
}

impl TryFrom<String> for AocCargoWorkspace {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        toml::from_str(value.as_str()).map_err(|e| anyhow!(format!("{e}")))
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
    let token = auth::get_token()?;
    let _client = AocClient::new(&token);

    let workspace = workspace_path()?;
    println!(
        "Creating solver for {year}.{day} under {}",
        workspace.display()
    );

    // Does the solvers dir exist yet?
    // todo: make solvers dir

    // Does this year's project exist yet?
    // todo: make project

    // Does this day already exist? (Abort if so)
    // todo: Check if day exists
    Ok(())
}

pub fn workspace_path() -> anyhow::Result<PathBuf> {
    std::env::current_dir()?
        .ancestors()
        .map(|dir| (dir, dir.join("Cargo.toml")))
        .filter_map(|(dir, file)| {
            if !(file.exists() && file.is_file()) { return None; }
            let Ok(cargo_toml) = std::fs::read_to_string(&file) else { return None; };
            let Ok(cargo_toml): anyhow::Result<AocCargoWorkspace> = cargo_toml.try_into() else { return None; };
            cargo_toml.is_rust_aoc_workspace().then_some(dir.into())
        })
        .next()
        .ok_or_else(|| anyhow!("No workspace found"))
}
