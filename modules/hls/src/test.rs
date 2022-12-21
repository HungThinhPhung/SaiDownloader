#[cfg(test)]
mod tests {
    use crate::get_response_bytes;

    #[test]
    fn send_request_work() {
        let url = "https://google.com";
        let x = get_response_bytes(url, None);
        assert_ne!(x.len(), 0);
    }
}
