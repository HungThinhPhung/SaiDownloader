pub use reqwest_impersonate::{header::HeaderMap, ClientBuilder, Response, Version};
use http::{header::HeaderName, HeaderValue};
use std::{fmt, str::FromStr, time::Duration};
use reqwest_impersonate::browser::ChromeVersion;
use serde::ser::Error;
use tokio::time;

// wrapped request is a request with retry and delay option
pub async fn send_wrapped_request(url: &str, headers: &Option<HeaderMap>, h2: bool, delay: Option<u64>, retry: Option<u8>) -> Result<Response, fmt::Error> {
    let mut response = send_request(url, headers, h2).await;
    if response.is_err() && retry.is_some() {
        let mut retry = retry.expect("Already checked");
        loop {
            // Wait 3 seconds before retry
            time::sleep(time::Duration::from_secs(3)).await;
            response = send_request(url, headers, h2).await;
            println!("Failed to send, retry times remaining: {}", retry);
            retry = retry - 1;
            if response.is_ok() || retry == 0 {
                break;
            }
        }
    }
    return match delay {
        Some(second) => {
            time::sleep(time::Duration::from_secs(second)).await;
            response
        }
        None => response
    };
}

pub async fn send_request(url: &str, headers: &Option<HeaderMap>, h2: bool) -> Result<Response, fmt::Error> {
    let client = ClientBuilder::new().chrome_builder(ChromeVersion::V106).build().unwrap();
    let mut req_builder = client.get(url);
    req_builder = req_builder.timeout(Duration::new(1000, 0));
    match headers {
        None => {}
        Some(headers) => {
            req_builder = req_builder.headers(headers.clone());
        }
    }
    println!("Downloading {}", url);
    if h2 {
        req_builder = req_builder.version(Version::HTTP_2);
    }
    return match req_builder.send().await {
        Ok(response) => {
            let status_code = response.status().as_u16();
            if status_code >= 400 {
                let err = fmt::Error::custom::<String>(format!("{} status code for {}", status_code, url).into());
                return Err(err);
            }
            Ok(response)
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(fmt::Error::custom::<String>("Failed to send request".to_string()));
        }
    };
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

        let header_name = split.next().expect("Line cannot be empty, checked when read file").trim();
        let header_value = split.next().unwrap_or_default().trim();

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