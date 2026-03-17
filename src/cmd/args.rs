use crate::cmd::metadata::{APP_DESCRIPTION, APP_NAME, APP_VERSION, INFO_TEXT};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = APP_NAME,
    version = APP_VERSION,
    about = APP_DESCRIPTION,
    after_help = INFO_TEXT
)]
pub struct Cli {
    #[arg(help = "HTTP method (e.g., GET, POST, PUT, DELETE, PATCH)")]
    pub method: String,

    #[arg(help = "Request URL")]
    pub url: String,

    #[arg(short, long, help = "Request body (for GET, POST, PUT, PATCH, DELETE)")]
    pub body: Option<String>,

    #[arg(
        short = 'H',
        long,
        help = "Custom headers (e.g., -H 'Authorization: Bearer token')"
    )]
    pub headers: Vec<String>,

    #[arg(short = 'q', long, help = "Query parameters (e.g., -q 'key=value')")]
    pub queries: Vec<String>,

    #[arg(
        short = 'a',
        long,
        help = "Bearer token for authorization (e.g., -a token or --bearer token)"
    )]
    pub bearer: Option<String>,

    #[arg(
        long,
        value_parser = ["basic", "bearer"],
        help = "Authentication scheme (basic or bearer)"
    )]
    pub auth: Option<String>,

    #[arg(
        long,
        help = "Auth credential value; bearer token or basic credentials as user:pass"
    )]
    pub auth_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_with_auth_and_bearer() {
        let args = Cli::parse_from([
            "http-req",
            "POST",
            "https://example.com/api",
            "--auth",
            "bearer",
            "--auth-value",
            "token123",
        ]);

        assert_eq!(args.method, "POST");
        assert_eq!(args.url, "https://example.com/api");
        assert_eq!(args.auth.unwrap(), "bearer");
        assert_eq!(args.auth_value.unwrap(), "token123");
    }

    #[test]
    fn parse_cli_with_a_bearer_alias() {
        let args = Cli::parse_from(["http-req", "GET", "https://example.com", "-a", "abc123"]);

        assert_eq!(args.method, "GET");
        assert_eq!(args.url, "https://example.com");
        assert_eq!(args.bearer.unwrap(), "abc123");
    }
}
