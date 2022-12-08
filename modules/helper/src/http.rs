pub use reqwest::{header::{HeaderMap, HeaderName, HeaderValue},
                  Client, Response};
use std::{fmt, str::FromStr, time::Duration};
use serde::ser::Error;

pub async fn send_request(url: &str, headers: &Option<HeaderMap>) -> Result<Response, fmt::Error> {
    let client = Client::new();
    let mut req_builder = client.get(url);
    req_builder = req_builder.timeout(Duration::new(1000, 0));
    match headers {
        None => {}
        Some(headers) => {
            req_builder = req_builder.headers(headers.clone());
        }
    }
    println!("Downloading {}", url);
    let response = req_builder.send().await.unwrap();
    let status_code = response.status().as_u16();
    if status_code >= 400 {
        let err = fmt::Error::custom::<String>(format!("{} status code for {}", status_code, url).into());
        return Err(err);
    }
    Ok(response)
}

pub fn lines_to_header(lines: impl Iterator<Item=String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for line in lines {
        // TODO: Handle pseudo-header http2 (tokio async)
        // Skip pseudo-header
        if line.starts_with(":") {
            continue
        }
        // Split string by first colon
        let mut split = line.splitn(2, ":");

        let header_name = split.next().expect("Line cannot be empty, checked when read file");
        let header_value = split.next().unwrap_or_default();

        // Skip no colon lines
        if header_value.is_empty() {
            continue
        }
        headers.append(
            HeaderName::from_str(header_name).unwrap(),
            HeaderValue::from_str(header_value).unwrap(),
        );
    }
    return headers;
}