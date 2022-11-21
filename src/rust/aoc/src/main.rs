use anyhow::Result;
use aoc::{gen, login};
use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(author = "@kyle-rader", about = "A cli for doing AOC.")]
enum Cli {
    /// Login via FireFox session token or pasting a token directly.
    Login { token: Option<String> },
    /// Generate a solver
    Gen { year: usize, day: usize },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Login { token } => login::login(token),
        Cli::Gen { year, day } => gen::new(year, day),
    }
}
