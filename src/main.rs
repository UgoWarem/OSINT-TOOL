use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send()?.text()?;
    Ok(res)
}

fn extract_links(html: &str) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            println!("Found link: {}", link);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.youtube.com";
    let html = fetch_html(url)?;
    extract_links(&html);
    Ok(())
}
