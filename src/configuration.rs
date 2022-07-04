use scraper::Html;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    pub index_uid: String,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub sitemap_urls: Vec<String>,
    #[serde(default)]
    pub start_urls: Vec<String>,
    pub selectors: Selectors,
    pub strip_chars: Option<String>,
    #[serde(default)]
    pub scrap_start_urls: bool,
    pub custom_settings: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selectors {
    pub lvl0: Option<Selector>,
    pub lvl1: Option<Selector>,
    pub lvl2: Option<Selector>,
    pub lvl3: Option<Selector>,
    pub lvl4: Option<Selector>,
    pub lvl5: Option<Selector>,
    pub lvl6: Option<Selector>,
    pub lvl7: Option<Selector>,
    pub lvl8: Option<Selector>,
    pub lvl9: Option<Selector>,
    pub text: Selector,
}

impl Selectors {
    pub fn scrape(&self, page: &Html) {
        match self.lvl0 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl1 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl2 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl3 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl4 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl5 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl6 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl7 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl8 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        match self.lvl9 {
            Some(ref selector) => selector.scrape(page),
            None => (),
        }

        self.text.scrape(page);
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Selector {
    Inlined(String),
    Full {
        selector: String,
        global: bool,
        default_value: String,
    },
}

impl Selector {
    pub fn scrape(&self, page: &Html) {
        let selector = match self {
            Selector::Inlined(s) => s,
            Selector::Full { selector, .. } => selector,
        };

        let selector = scraper::Selector::parse(selector).expect("could not parse selector");
        let res = page.select(&selector);
        println!("Got {res:?} from selector {selector:?}");
    }
}
