pub fn login(token: Option<String>) -> anyhow::Result<()> {
    // Get
    let token = token.unwrap_or(token_from_cookies()?);
    println!("Using token: {token}");
    // Test
    // Save
    Ok(())
}

fn token_from_cookies() -> anyhow::Result<String> {
    Ok(String::from("foobar"))
}
