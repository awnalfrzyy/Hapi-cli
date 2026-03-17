use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

pub fn auth(
    headers: &mut HashMap<String, String>,
    auth_mode: Option<String>,
    auth_value: Option<String>,
) {
    let auth_mode = auth_mode.map(|v| v.to_lowercase());

    match (auth_mode.as_deref(), auth_value) {
        (Some("bearer"), Some(token)) => {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        (Some("basic"), Some(credentials)) => {
            let creds = general_purpose::STANDARD.encode(credentials);
            headers.insert("Authorization".to_string(), format!("Basic {}", creds));
        }
        (None, Some(token)) => {
            headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_auth_inserts_header() {
        let mut headers = HashMap::new();
        auth(&mut headers, Some("bearer".into()), Some("abc123".into()));
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer abc123".to_string())
        );
    }

    #[test]
    fn basic_auth_inserts_encoded_header() {
        let mut headers = HashMap::new();
        auth(&mut headers, Some("basic".into()), Some("user:pass".into()));
        let expected = format!("Basic {}", general_purpose::STANDARD.encode("user:pass"));
        assert_eq!(headers.get("Authorization"), Some(&expected));
    }

    #[test]
    fn no_auth_mode_with_token_uses_bearer() {
        let mut headers = HashMap::new();
        auth(&mut headers, None, Some("abc123".into()));
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer abc123".to_string())
        );
    }

    #[test]
    fn missing_auth_mode_and_value_does_nothing() {
        let mut headers = HashMap::new();
        auth(&mut headers, None, None);
        assert!(headers.is_empty());
    }
}
