use anyhow::anyhow;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub struct AocClient<'a> {
    token: &'a String,
    client: Client,
}

impl<'a> AocClient<'a> {
    pub fn new(token: &'a String) -> Self {
        let client = reqwest::blocking::Client::new();
        AocClient { token, client }
    }

    pub fn user_name(&self) -> anyhow::Result<String> {
        let resp = self
            .client
            .get("https://adventofcode.com")
            .header(reqwest::header::COOKIE, format!("session={}", self.token))
            .send()?
            .text()?;

        let doc = Html::parse_fragment(resp.as_str());
        let user_selector = Selector::parse("div.user").expect("bad selector");

        Ok(doc
            .select(&user_selector)
            .next()
            .ok_or_else(|| anyhow!("It doesn't look like that token worked 🤔"))?
            .inner_html())
    }
}
