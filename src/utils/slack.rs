use reqwest::{self, Error, Response};
use std::str;

pub async fn send_webhook_message(host: &str, message: &str) -> Result<Response, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert("text", message);

    client.post(host).json(&map).send().await
}

#[cfg(test)]
mod tests {
    use super::send_webhook_message;

    #[tokio::test]
    async fn test_send_webhook_message() {
        let host: &str = "";
        let _result = send_webhook_message(host, "test").await;
    }
}
