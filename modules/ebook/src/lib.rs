use std::fmt::{Debug, Display};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub flow: EbookFlow,
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

pub struct EbookData<U: Display> {
    title: U,
    content: U,
}

trait StandardFlow<T, U: Display>: Output<U> {
    type Data: IntoIterator<Item=EbookData<U>>;

    fn download_contents(config: T) -> Self::Data;
}

trait Output<U: Display> {
    fn write_output(titles: &dyn Iterator<Item=U>, content: &dyn Iterator<Item=U>) {}
}

pub struct IterationFlow;

pub struct TocFlow;

impl Output<String> for TocFlow {}

impl StandardFlow<TocConfig, String> for TocFlow {
    type Data = Vec<EbookData<String>>;

    fn download_contents(config: TocConfig) -> Self::Data {
        vec![EbookData { title: "".to_string(), content: "".to_string() }]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
