use scraper::{ElementRef, Html, Selector};
use select::document::Document;
use select::predicate::Name;
use crate::Chapter;

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

pub fn get_text_from_selector(document: &Html, selector: &str) -> String {
    let mut result = String::new();
    let first_select_result = get_first_selection(&document, selector);
    for item in first_select_result.text() {
        result.push_str(item);
    }
    return result;
}

pub fn get_url_from_selector(document: &Html, selector: &str) -> Option<String> {
    let first_select_result = get_first_selection(&document, selector);
    let url = first_select_result.value().attr("href");
    match url {
        Some(u) => Some(u.to_string()),
        None => None,
    }
}

fn get_first_selection<'a>(document: &'a Html, selector: &str) -> ElementRef<'a> {
    let parsed_selector = Selector::parse(selector).unwrap();
    document.select(&parsed_selector).next().unwrap()
}

pub async fn single_page_extract(document: &Html, title_selector: &str, content_selector: &str) -> Chapter<String> {
    let title = get_text_from_selector(document, title_selector);
    let content = get_text_from_selector(document, content_selector);
    Chapter { title, content }
}

pub async fn single_page_extract_with_next_url(document: &Html, title_selector: &str, content_selector: &str, next_url_selector: &str) -> (Chapter<String>, Option<String>) {
    let page_content = single_page_extract(document, title_selector, content_selector).await;
    let next_url = get_url_from_selector(document, next_url_selector);
    (page_content, next_url)
}