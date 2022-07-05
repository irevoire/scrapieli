use scraper::Html;
use serde::Deserialize;
use serde_json::Value;
use spider::page::Page;
use uuid::Uuid;

use crate::output::Output;

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
    pub fn scrape(&self, page: &Page) -> Output {
        let html = Html::parse_document(&page.get_html());

        Output {
            hierarchy_radio_lvl0: None,
            hierarchy_radio_lvl1: None,
            hierarchy_radio_lvl2: None,
            hierarchy_radio_lvl3: None,
            hierarchy_radio_lvl4: None,
            hierarchy_radio_lvl5: None,
            hierarchy_lvl0: self.lvl0.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl1: self.lvl1.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl2: self.lvl2.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl3: self.lvl3.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl4: self.lvl4.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl5: self.lvl5.as_ref().map(|selector| selector.scrape(&html)),
            hierarchy_lvl6: self.lvl6.as_ref().map(|selector| selector.scrape(&html)),
            content: self.text.scrape(&html),
            object_id: Uuid::new_v4().to_string(),
            anchor: get_anchor(self.text.selector(), &html),
            url: page.get_url().clone(),
        }
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
    pub fn selector(&self) -> &str {
        match self {
            Selector::Inlined(s) => s.as_ref(),
            Selector::Full { selector, .. } => selector.as_ref(),
        }
    }

    pub fn scrape(&self, page: &Html) -> String {
        let selector = self.selector();
        let selector = scraper::Selector::parse(selector).expect("could not parse selector");
        let mut res = page.select(&selector);
        match res.next() {
            Some(node) => node.text().fold(String::new(), |acc, elem| acc + elem),
            None => String::new(),
        }
    }
}

fn get_anchor(selector: &str, page: &Html) -> Option<String> {
    let selector = scraper::Selector::parse(selector).expect("could not parse selector");
    let mut res = page.select(&selector);

    let node = match res.next() {
        Some(node) => node,
        None => return None,
    };

    if let Some(tag) = node.value().id() {
        return Some(tag.to_string());
    }

    let mut iter = node.ancestors();

    loop {
        match iter.next() {
            None => return None,
            Some(node) => {
                if let Some(tag) = node.value().as_element().and_then(|el| el.id()) {
                    return Some(tag.to_string());
                }
            }
        }
    }
}
