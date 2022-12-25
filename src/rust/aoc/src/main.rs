use anyhow::Result;
use aoc::{auth, gen};
use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(author = "@kyle-rader", about = "A cli for doing AOC.")]
enum Cli {
    /// Login via FireFox session token or pasting a token directly.
    Login { token: Option<String> },
    /// Clear the token cache.
    Logout,
    /// Generate a solver
    Gen {
        #[arg(value_parser = clap::value_parser!(u16).range(2015..))]
        year: u16,
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: u8,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Login { token } => auth::login(token),
        Cli::Logout => auth::logout(),
        Cli::Gen { year, day } => gen::new(year as usize, day as usize),
    }
}
