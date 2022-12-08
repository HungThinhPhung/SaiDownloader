mod dom;

use std::fmt;
use std::fmt::Display;
use std::fs::File;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use scraper::{Html};
use saidl_helper::http::{HeaderMap, send_request};

use serde::Deserialize;
use crate::dom::{single_page_extract, single_page_extract_with_next_url};


#[derive(Deserialize)]
pub struct Config {
    pub flow: EbookFlow,
    pub name: String,
    pub title_selector: String,
    pub content_selector: String,
}

#[derive(Deserialize)]
pub struct IterationConfig {
    pub base_url: String,
    pub next_selector: String,
    pub stop_url: String,

    // In case of relative href
    pub relative_base: Option<String>,
}

#[derive(Deserialize)]
pub struct TocConfig {
    base_url: String,
    toc_selector: String,
    void_sub: String,
}

#[derive(Deserialize)]
pub struct NumConfig {
    pattern: String,
    start: u16,
    end: u16,
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

    // Have an url pattern and a number replacer
    Num(NumConfig)
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

    fn write(self, chapter_num: bool) -> Result<(), std::fmt::Error>;
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

    fn write(self, chapter_num: bool) -> Result<(), std::fmt::Error>{
        let mut file = File::create(self.book_name.to_owned() + ".epub").unwrap();
        let mut ebook_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
        ebook_builder.metadata("author", "Sai").unwrap().metadata("title", &self.book_name).unwrap();
        for (id, chapter) in self.content.into_iter().enumerate() {
            let Chapter { mut title, content } = chapter;
            if chapter_num {
                title = format!("Chapter {}: {}", id + 1, title);
            }
            let content = content_to_xhtml(&title, &content);
            ebook_builder
                .add_content(EpubContent::new(format!("{}.xhtml", id), content.as_bytes())
                    .title(&title)
                    .reftype(ReferenceType::TitlePage)).unwrap();
        }
        ebook_builder.inline_toc().generate(&mut file).unwrap();
        Ok(())
    }
}

pub struct EBConfig<'a> {
    pub title_selector: String,
    pub content_selector: String,
    pub h2: bool,
    pub headers: &'a Option<HeaderMap>
}

pub struct IterDownloader {
    config: IterationConfig
}

pub struct NumDownloader {
    config: NumConfig,
}

pub struct TocDownloader {
    config: TocConfig
}

pub async fn single_page_download(url: &str, headers: &Option<HeaderMap>, h2: bool) -> Html {
    let response = send_request(&url, headers, h2).await.unwrap();
    let raw_html = response.text().await.unwrap();
    dom::get_dom(&raw_html)
}

impl IterDownloader {
    pub fn build(config: IterationConfig) -> Self {
        Self {
            config
        }
    }

    pub async fn download(self, cli_config: EBConfig<'_>) -> StandardContent {
        let mut result = Vec::new();
        let mut url = self.config.base_url;
        let next_selector = self.config.next_selector;
        loop {
            let document = single_page_download(&url, cli_config.headers, cli_config.h2).await;
            let (page_content, next_url) = single_page_extract_with_next_url(&document, &cli_config.title_selector, &cli_config.content_selector, &next_selector).await;
            result.push(page_content);
            if url == self.config.stop_url {
                break
            }
            url = next_url.unwrap();
        }
        result
    }
}

impl NumDownloader {
    pub fn build(config: NumConfig) -> Self {
        Self {
            config
        }
    }

    pub async fn download(self, cli_config: EBConfig<'_>) -> StandardContent {
        let mut result = Vec::new();
        for number in self.config.start..=self.config.end {
            let url = self.config.pattern.replace("$", &number.to_string());
            let document = single_page_download(&url, cli_config.headers, cli_config.h2).await;
            let page_content = single_page_extract(&document, &cli_config.title_selector, &cli_config.content_selector).await;
            result.push(page_content);
        }
        result
    }
}

impl TocDownloader {
    pub fn build(config: TocConfig) -> Self {
        Self {
            config
        }
    }

    pub async fn download(self, cli_config: EBConfig<'_>) -> StandardContent {
        let mut result = Vec::new();
        let links = self.extract_links(cli_config.headers, cli_config.h2).await.unwrap();
        for link in links {
            let document = single_page_download(&link, cli_config.headers, cli_config.h2).await;
            let page_content = single_page_extract(&document, &cli_config.title_selector, &cli_config.content_selector).await;
            result.push(page_content);
        }
        result
    }

    async fn extract_links(&self, headers: &Option<HeaderMap>, h2: bool) -> Result<Vec<String>, fmt::Error> {
        let base_page_response = send_request(&self.config.base_url, headers, h2).await?;
        // In case of toc is a dedicate request
        let result = if self.config.toc_selector == "" {
            let urls = dom::get_all_urls(&base_page_response.text().await.unwrap(), &self.config.void_sub);
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
        assert!(writer.write(false).is_ok());
        assert!(fs::remove_file("TestBook.epub").is_ok());
    }
}
