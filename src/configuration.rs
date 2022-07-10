use scraper::{html::Select, ElementRef, Html};
use serde::{de, Deserialize};
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
    pub text: Selector,
}

impl Selectors {
    pub fn scrape(&self, page: &Page) -> Output {
        let html = Html::parse_document(&page.get_html());

        let base_output = Output {
            hierarchy_radio_lvl0: None,
            hierarchy_radio_lvl1: None,
            hierarchy_radio_lvl2: None,
            hierarchy_radio_lvl3: None,
            hierarchy_radio_lvl4: None,
            hierarchy_radio_lvl5: None,
            hierarchy_lvl0: None,
            hierarchy_lvl1: None,
            hierarchy_lvl2: None,
            hierarchy_lvl3: None,
            hierarchy_lvl4: None,
            hierarchy_lvl5: None,
            hierarchy_lvl6: None,
            content: self
                .text
                .scrape(&html)
                .take(1)
                .map(|elem_ref| element_ref_to_string(&elem_ref))
                .collect::<String>(),
            object_id: Uuid::new_v4().to_string(),
            anchor: get_anchor(self.text.selector(), &html),
            url: page.get_url().clone(),
        };

        base_output
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Selector {
    Inlined(#[serde(deserialize_with = "deserialize_selector")] scraper::Selector),
    Full {
        #[serde(deserialize_with = "deserialize_selector")]
        selector: scraper::Selector,
        global: bool,
        default_value: String,
    },
}

impl Selector {
    pub fn selector(&self) -> &scraper::Selector {
        match self {
            Selector::Inlined(s) => s,
            Selector::Full { selector, .. } => selector,
        }
    }

    pub fn scrape<'a, 'b>(&'a self, page: &'b Html) -> Select<'b, 'a> {
        let selector = self.selector();
        page.select(selector)
    }
}

fn get_anchor(selector: &scraper::Selector, page: &Html) -> Option<String> {
    let mut res = page.select(selector);

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

fn element_ref_to_string(element_ref: &ElementRef) -> String {
    element_ref
        .text()
        .fold(String::new(), |acc, elem| acc + elem)
}

fn deserialize_selector<'de, D>(deserializer: D) -> Result<scraper::Selector, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    Ok(scraper::Selector::parse(s).expect("Couldn’t parse selector"))
}
