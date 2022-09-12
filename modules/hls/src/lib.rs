use bytes::Bytes;

pub fn send_request(url: &str) -> Bytes {
    let response = reqwest::blocking::get(url).expect(&format!("Send request failed: {}", url)[..]);
    let data = response.bytes().expect(&format!("Unpack data failed: {}", url)[..]);
    return data;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
