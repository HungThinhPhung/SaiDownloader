pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

pub fn lines_to_header(lines: &Vec<String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for line in lines {
        // TODO: Handle pseudo-header http2 (tokio async)
        // Skip pseudo-header
        if line.starts_with(":") {
            continue
        }
        // Split string by first colon
        let split: Vec<&str> = line.splitn(2, ":").collect();

        // Skip no colon line
        if !(split.len() == 2) { continue };
        let mut temp_value = split[1];
        if temp_value.ends_with("\r") {
            temp_value = &temp_value[..temp_value.len()-1];
        }

        let header_name = HeaderName::from_str(split[0]).unwrap();
        let header_value = HeaderValue::from_str(temp_value).unwrap();
        headers.append(header_name, header_value);
    }
    return headers;
}