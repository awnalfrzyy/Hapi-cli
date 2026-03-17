mod cli;
mod http;

use clap::Parser;
use cli::Cli;
use colored::*;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

const LOGO: &str = r#"
       __    __   __                       
      / /_  / /_ / /_ ____   -  ____  ___   ____ _ 
     / __ \/ __// __// __ \ / / / __ \/ _ \ / __ `/ 
    / / / / /_ / /_ / /_/ // / / /_/ /  __// /_/ /  
   /_/ /_/\__/ \__/ / .___//_/ \____/\___/ \__, /   
                   /_/    BUILD BY DIGGIE /____/    
"#;

fn print_logo() {
    println!("{}", LOGO.cyan().bold());
    println!("{} Minimalist CLI HTTP Client", "v0.1.0".yellow());
    println!("--------------------------------------------------");
}

#[tokio::main]
async fn main() {
    let raw_args: Vec<String> = env::args().collect();

    if raw_args.len() == 1 {
        print_logo();
        println!(
            "{} {}",
            "{} http-req <METHOD> <URL> [OPTIONS]",
            "Usage:".yellow(),
        );
        println!("Run {} for more details.\n", "--help".green());
        return;
    }

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
        &client,
        &args.method,
        &args.url,
        args.body,
        headers_map,
        queries_map,
    );

    match rb {
        Ok(req) => {
            println!(
                "{} {} sending request to {}...",
                "🚀".cyan(),
                args.method.to_uppercase().bold(),
                args.url.underline()
            );

            let start = Instant::now();

            match req.send().await {
                Ok(res) => {
                    let duration = start.elapsed();

                    if let Err(e) = http::response::print_response(res).await {
                        eprintln!("{} {}", "Error printing response:".red(), e);
                    }

                    println!("\n{} {:?}", "⚡ Time Elapsed:".bold().blue(), duration);
                }
                Err(e) => eprintln!("{} {}", "Request failed:".red(), e),
            }
        }
        Err(e) => eprintln!("{} {}", "Configuration error:".red(), e),
    }
}
