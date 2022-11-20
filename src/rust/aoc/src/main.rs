use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[command(author = "@kyle-rader", about = "A cli for doing AOC.")]
enum Cli {
    /// login via FireFox session token or pasting a token directly.
    Login { token: Option<String> },
    /// generate a solver
    Generate { year: usize, day: usize },
}

fn main() {
    let args = Cli::parse();
    match args {
        Cli::Login { token } => {
            println!(
                "Login with token: {}",
                token.unwrap_or_else(|| String::from("<not given>"))
            );
        }
        Cli::Generate { year, day } => {
            println!("Generate year {year} day {day}");
        }
    }
}
