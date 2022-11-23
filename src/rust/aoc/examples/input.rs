use aoc::{aoc_client::AocClient, auth::get_token};

fn main() {
    let token = get_token().expect("could not get token");
    let client = AocClient::new(&token);

    let year = 2015;
    let day = 1;

    match client.problem(year, day) {
        Ok(problem) => println!("Got problem:\n{problem}"),
        Err(e) => println!("Error: {e}"),
    }

    match client.input(year, day) {
        Ok(input) => println!("Got input:\n{input}"),
        Err(e) => println!("Error: {e}"),
    }
}
