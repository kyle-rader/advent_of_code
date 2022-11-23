use anyhow::anyhow;
use reqwest::{blocking::Client, IntoUrl};
use scraper::{Html, Selector};

const AOC_URL: &'static str = "https://adventofcode.com";

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

        Ok(doc
            .select(&user_selector)
            .next()
            .ok_or_else(|| anyhow!("It doesn't look like that token worked 🤔"))?
            .inner_html())
    }

    pub fn input(&self, year: usize, day: usize) -> anyhow::Result<String> {
        self.get(format!("{AOC_URL}/{year}/day/{day}/input"))
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
