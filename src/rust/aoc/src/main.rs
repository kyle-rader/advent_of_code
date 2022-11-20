mod cookies;
mod gen;
mod login;
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(author = "@kyle-rader", about = "A cli for doing AOC.")]
enum Cli {
    /// login via FireFox session token or pasting a token directly.
    Login { token: Option<String> },
    /// generate a solver
    Generate { year: usize, day: usize },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Login { token } => login::login(token),
        Cli::Generate { year, day } => gen::new(year, day),
    }
}
