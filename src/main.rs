mod http;
mod cli;

use clap::Parser;
use cli::Cli;
use std::collections::HashMap;
use colored::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let client = http::request::build_client();

    let mut headers_map = HashMap::new();
    for h in args.headers {
        if let Some((k, v)) = h.split_once(':') {
            headers_map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let mut queries_map = HashMap::new();
    for q in args.queries {
        if let Some((k, v)) = q.split_once('=') {
            queries_map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let rb = http::request::create_request(
        &client, &args.method, &args.url, args.body, headers_map, queries_map
    );

    match rb {
        Ok(req) => {
            println!("{} sending request...", "🚀".cyan());
            match req.send().await {
                Ok(res) => {
                    if let Err(e) = http::response::print_response(res).await {
                        eprintln!("{} {}", "Error printing response:".red(), e);
                    }
                },
                Err(e) => eprintln!("{} {}", "Request failed:".red(), e),
            }
        },
        Err(e) => eprintln!("{} {}", "Configuration error:".red(), e),
    }
}