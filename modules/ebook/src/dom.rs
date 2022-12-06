use scraper::{Html, Selector};
use scraper::html::Select;
use select::document::Document;
use select::predicate::Name;
use serde::de::Unexpected::Str;

const CURRENT_URLS: [&'static str; 2] = ["javascript:void(0);", "#"];

pub fn get_dom(raw_html: &str) -> Html {
    return Html::parse_document(raw_html);
}

pub fn get_all_urls(raw_html: &str, void_sub: &str) -> Vec<String> {
    Document::from(raw_html).find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map({ |x| {
            if CURRENT_URLS.contains(&x.trim()) {
                print!("{x}");
                return String::from(void_sub);
            }
            String::from(x)
        } }).collect()
}
