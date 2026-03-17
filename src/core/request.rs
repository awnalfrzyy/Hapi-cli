use reqwest::{Client, Method, RequestBuilder};
use std::collections::HashMap;
use std::time::Duration;
use urlencoding;

pub fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

pub fn create_request(
    client: &Client,
    method_str: &str,
    url: &str,
    body: Option<String>,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
) -> Result<RequestBuilder, String> {
    let method = match method_str.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        _ => return Err(format!("Method '{}' nggak dikenal.", method_str)),
    };

    let mut full_url = url.to_string();
    if !queries.is_empty() {
        let query_pairs: Vec<String> = queries
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect();

        let separator = if full_url.contains('?') { '&' } else { '?' };
        full_url.push(separator);
        full_url.push_str(&query_pairs.join("&"));
    }

    let mut rb = client.request(method, full_url);

    for (k, v) in &headers {
        rb = rb.header(k, v);
    }

    if let Some(b) = body {
        let json_value: serde_json::Value =
            serde_json::from_str(&b).map_err(|e| format!("Body bukan JSON valid: {}", e))?;
        rb = rb.json(&json_value);
    }

    Ok(rb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_get_with_query_header() {
        let client = build_client();
        let mut headers = HashMap::new();
        headers.insert("X-Test".to_string(), "true".to_string());

        let mut queries = HashMap::new();
        queries.insert("q".to_string(), "1".to_string());

        let rb = create_request(
            &client,
            "GET",
            "https://example.com/api",
            None,
            headers.clone(),
            queries.clone(),
        )
        .unwrap();
        let req = rb.build().unwrap();

        assert_eq!(req.method(), "GET");
        assert_eq!(req.url().as_str(), "https://example.com/api?q=1");
        assert_eq!(req.headers().get("x-test").unwrap(), "true");
    }

    #[test]
    fn create_request_body_json_must_parse() {
        let client = build_client();
        let headers = HashMap::new();
        let queries = HashMap::new();

        let rb = create_request(
            &client,
            "POST",
            "https://example.com/api",
            Some(r#"{"a":1}"#.to_string()),
            headers,
            queries,
        )
        .unwrap();

        let req = rb.build().unwrap();
        assert_eq!(req.method(), "POST");
        assert_eq!(req.url().as_str(), "https://example.com/api");
    }

    #[test]
    fn create_request_body_json_invalid_returns_error() {
        let client = build_client();
        let headers = HashMap::new();
        let queries = HashMap::new();

        let err = create_request(
            &client,
            "POST",
            "https://example.com/api",
            Some("not-json".to_string()),
            headers,
            queries,
        )
        .unwrap_err();

        assert!(err.contains("Body bukan JSON valid"));
    }

    #[test]
    fn create_request_invalid_method_returns_error() {
        let client = build_client();
        let headers = HashMap::new();
        let queries = HashMap::new();

        let err = create_request(
            &client,
            "INVALID",
            "https://example.com",
            None,
            headers,
            queries,
        )
        .unwrap_err();
        assert!(err.contains("nggak dikenal"));
    }
}
