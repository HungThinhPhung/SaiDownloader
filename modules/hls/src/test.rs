#[cfg(test)]
mod tests {
    use crate::send_request;

    #[test]
    fn send_request_work() {
        let url = "https://google.com";
        let x = send_request(url);
        assert_ne!(x.len(), 0);
    }
}