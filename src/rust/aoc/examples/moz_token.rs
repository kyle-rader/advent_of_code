use aoc::cookies::{aoc_session_token, firefox_cookies_paths};

fn main() {
    let cookie_paths = firefox_cookies_paths().expect("no cookies!");
    let token = aoc_session_token(&cookie_paths[0]).expect("no session token!");
    println!("Got: {token}");
}
