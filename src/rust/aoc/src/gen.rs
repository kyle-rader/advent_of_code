use anyhow::Result;

use crate::{aoc_client::AocClient, auth};

pub fn new(_year: usize, _day: usize) -> Result<()> {
    let token = auth::get_token()?;
    let _client = AocClient::new(&token);
    Ok(())
}
