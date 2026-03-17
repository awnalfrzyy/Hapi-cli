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
        _ => {
            return Err(format!(
                "Method '{}' nggak dikenal. Pake GET, POST, PUT, DELETE, atau PATCH.",
                method_str
            ))
        }
    };

    let mut full_url = url.to_string();
    if !queries.is_empty() {
        let query_string = queries
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        full_url = format!("{}?{}", url, query_string);
    }

    let mut rb = client.request(method, &full_url);

    for (k, v) in &headers {
        rb = rb.header(k, v);
    }

    if let Some(b) = body {
        let json_value: serde_json::Value =
            serde_json::from_str(&b).map_err(|e| format!("Body bukan JSON yang valid: {}", e))?;
        rb = rb.json(&json_value);
    }

    Ok(rb)
}
