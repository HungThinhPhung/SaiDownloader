mod dom;

use std::fmt;
use std::fmt::{Debug, Display, format};
use std::fs::File;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use scraper::Selector;
use saidl_helper::http::send_request;

use serde::Deserialize;


#[derive(Deserialize)]
pub struct Config {
    pub flow: EbookFlow,
    pub name: String,
    pub title_selector: String,
    pub content_selector: String,
}

#[derive(Deserialize)]
pub struct IterationConfig {
    pub iter: String,
}

#[derive(Deserialize)]
pub struct TocConfig {
    base_url: String,
    toc_selector: String,
    void_sub: String,
}

#[derive(Deserialize)]
#[serde(tag = "mode", content = "args", rename_all = "snake_case")]
pub enum EbookFlow {
    // Set up initial url, download initial content
    // Extract next url from initial content
    // Run util stop condition is matched
    Iter(IterationConfig),

    // Extract all url from a html element, which contains table of content
    Toc(TocConfig),
}

pub struct Chapter<T: Display> {
    pub title: T,
    pub content: T,
}

pub trait WriteBook<T, U> where
    T: Display,
    U: IntoIterator<Item=Chapter<T>>,
{
    fn build(book_name: T, content: U) -> Self;

    fn write(self) -> Result<(), std::fmt::Error>;
}

pub type StandardContent = Vec<Chapter<String>>;

pub struct StandardEpub {
    book_name: String,
    content: StandardContent,
}

impl WriteBook<String, StandardContent> for StandardEpub {
    fn build(book_name: String, content: StandardContent) -> Self {
        Self {
            book_name,
            content
        }
    }

    fn write(self) -> Result<(), std::fmt::Error>{
        let mut file = File::create(self.book_name.to_owned() + ".epub").unwrap();
        let mut ebook_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
        ebook_builder.metadata("author", "Sai").unwrap().metadata("title", &self.book_name).unwrap();
        for (id, chapter) in self.content.into_iter().enumerate() {
            let content = content_to_xhtml(&chapter.title,&chapter.content);
            ebook_builder
                .add_content(EpubContent::new(format!("{}.xhtml", id), content.as_bytes())
                    .title(&chapter.title.to_string())
                    .reftype(ReferenceType::TitlePage)).unwrap();
        }
        ebook_builder.inline_toc().generate(&mut file).unwrap();
        Ok(())
    }
}

pub struct TocDownloader {
    config: TocConfig,
    title_selector: String,
    content_selector: String,
}

pub struct SinglePageContent {
    next_url: Option<String>,
    title: String,
    content: String,
}

impl SinglePageContent {
    pub fn to_chapter(self) -> Chapter<String> {
        Chapter { title: self.title, content: self.content }
    }
}

pub fn single_page_download(url: &str, title_selector: &str, content_selector: &str, next_url_selector: Option<&str>) -> SinglePageContent {
    let response = send_request(&url, &None).unwrap();
    let raw_html = response.text().unwrap();
    let document = dom::get_dom(&raw_html);
    let title_selector = Selector::parse(title_selector).unwrap();
    let content_selector = Selector::parse(content_selector).unwrap();
    let title_raw_data = document.select(&title_selector).next().unwrap();
    let content_raw_data = document.select(&content_selector).next().unwrap();
    let next_url: Option<String> = match next_url_selector {
        // TODO: Handle this case (only for iteration flow)
        Some(url) => { Some("".to_string()) },
        None => None
    };
    let (mut title, mut content) = (String::new(), String::new());
    for t in title_raw_data.text() {
        title.push_str(t);
    }
    for c in content_raw_data.text() {
        content.push_str(c);
    }
    return SinglePageContent { next_url, title, content}
}

impl TocDownloader {
    pub fn build(config: TocConfig, title_selector: String, content_selector: String) -> Self {
        Self {
            config,
            title_selector,
            content_selector
        }
    }

    pub fn download(self) -> StandardContent {
        let mut result = Vec::new();
        let links = self.extract_links().unwrap();
        for link in links {
            let page_content = single_page_download(&link, &self.title_selector, &self.content_selector, None);
            result.push(page_content.to_chapter());
        }
        result
    }

    fn extract_links(&self) -> Result<Vec<String>, fmt::Error> {
        let base_page_response = send_request(&self.config.base_url, &None)?;
        // In case of toc is a dedicate request
        let result = if self.config.toc_selector == "" {
            let urls = dom::get_all_urls(&base_page_response.text().unwrap(), &self.config.void_sub);
            urls
        } else {
            // TODO: In case of toc is not a dedicate request
            vec![]
        };
        Ok(result)
    }
}

fn content_to_xhtml(title: &str, content: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="https://www.w3.org/ns/epub/2007/ops/"><body>
    <h1>{}</h1>{}</body></html>"#, title, content)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{Chapter, StandardEpub, WriteBook};
    #[test]
    fn write_standard_epub_ok() {
        let content = vec![
            Chapter { title: "tt".to_string(), content: "ct".to_string() },
            Chapter { title: "tt2".to_string(), content: "ct2".to_string() },
            Chapter { title: "tt3".to_string(), content: "ct3".to_string() },
        ];

        let writer = StandardEpub::build("TestBook".to_string(), content);
        assert!(writer.write().is_ok());
        assert!(fs::remove_file("TestBook.epub").is_ok());
    }
}
