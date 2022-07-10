mod configuration;
mod output;

use configuration::Configuration;
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

    let mut website = Website::new(&conf.start_urls[0]);
    website.scrape();

    for page in website.get_pages() {
        let output = conf.selectors.scrape(&page);
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
        break;
    }
}
