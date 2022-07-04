mod configuration;

use configuration::Configuration;
use scraper::Html;
use spider::website::Website;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage:");
        eprintln!("\t{} [configuration file]", args[0]);
        return;
    }

    let conf_file = std::fs::read_to_string(&args[1]).unwrap();
    let conf: Configuration = serde_json::from_str(&conf_file).unwrap();

    println!("starting to crawl");
    let mut website = Website::new(&conf.start_urls[0]);
    website.on_link_find_callback = |url| {
        println!("encountered {url}");
        url
    };
    website.scrape();
    println!("scraped");

    for page in website.get_pages() {
        println!("crawling on {}", page.get_url());

        let page = page.get_html();
        let html = Html::parse_document(&page);

        conf.selectors.scrape(&html);
    }
}
