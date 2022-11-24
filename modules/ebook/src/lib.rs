use std::fmt::{Debug, Display, format};
use std::fs::File;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};

use serde::Deserialize;

const CURRENT_URLS: [&'static str; 2] = ["javascript:void(0)", "#"];

#[derive(Deserialize)]
pub struct Config {
    pub flow: EbookFlow,
    pub name: String,
}

#[derive(Deserialize)]
pub struct IterationConfig {
    pub iter: String,
}

#[derive(Deserialize)]
pub struct TocConfig {
    base_url: String,
    selector: String,
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

pub struct Chapter<U: Display> {
    pub title: U,
    pub content: U,
}

pub trait StandardFlow<T, U: Display>: Output<U> {
    type Data: IntoIterator<Item=Chapter<U>>;

    fn download_contents(config: T) -> Self::Data;

    fn execute(config: T) {
        let mut data = Self::download_contents(config);
        Self::write_output(&mut data.into_iter()).unwrap();
    }
}

pub trait Output<U: Display> {
    fn write_output(data: &mut dyn Iterator<Item=Chapter<U>>) -> Result<(), std::fmt::Error> {
        let mut file = File::create("result.epub").unwrap();
        let mut ebook_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
        ebook_builder.metadata("author", "Sai").unwrap().metadata("title", "Dummy Book").unwrap();
        for (id, chapter) in data.enumerate() {
            let content = content_to_xhtml(&chapter.content.to_string());
            ebook_builder
                .add_content(EpubContent::new(format!("{}.xhtml", id), content.as_bytes())
                    .title(&chapter.title.to_string())
                    .reftype(ReferenceType::TitlePage)).unwrap();
        }
        ebook_builder.inline_toc().generate(&mut file).unwrap();
        Ok(())
    }
}

pub struct IterationFlow;

pub struct TocFlow;

impl Output<String> for TocFlow {}

impl StandardFlow<TocConfig, String> for TocFlow {
    type Data = Vec<Chapter<String>>;

    fn download_contents(config: TocConfig) -> Self::Data {
        vec![
            Chapter { title: "tt".to_string(), content: "ct".to_string() },
            Chapter { title: "tt2".to_string(), content: "ct2".to_string() },
            Chapter { title: "tt3".to_string(), content: "ct3".to_string() },
        ]
    }
}

fn content_to_xhtml(content: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
    <html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops"><body>
    {}</body></html>"#, content)
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
