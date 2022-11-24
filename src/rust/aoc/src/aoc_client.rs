use anyhow::anyhow;
use reqwest::{blocking::Client, IntoUrl};
use scraper::{Html, Selector};

const AOC_URL: &str = "https://adventofcode.com";

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
        let resp = self.get(AOC_URL)?;

        let doc = Html::parse_fragment(resp.as_str());
        let user_selector = Selector::parse("div.user").expect("bad selector");

        doc.select(&user_selector)
            .next()
            .ok_or_else(|| anyhow!("It doesn't look like that token worked 🤔"))?
            .text()
            .next()
            .map(|text| text.trim().to_string())
            .ok_or_else(|| anyhow!("Could not find username in html output!"))
    }

    pub fn input(&self, year: usize, day: usize) -> anyhow::Result<String> {
        self.get(format!("{AOC_URL}/{year}/day/{day}/input"))
    }

    pub fn problem(&self, year: usize, day: usize) -> anyhow::Result<String> {
        self.get(format!("{AOC_URL}/{year}/day/{day}"))
    }

    fn get<U>(&self, url: U) -> anyhow::Result<String>
    where
        U: IntoUrl,
    {
        match self
            .client
            .get(url)
            .header(reqwest::header::COOKIE, format!("session={}", self.token))
            .send()
        {
            Ok(resp) => resp.text().map_err(|e| anyhow!(format!("{e}"))),
            Err(e) => Err(anyhow!(format!("{e}"))),
        }
    }
}
