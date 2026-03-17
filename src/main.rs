use clap::{CommandFactory, Parser};
use colored::*;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

use tooling::assets;
use tooling::cmd::args::Cli;
use tooling::core;

fn print_logo() {
    println!("{}", assets::logo::LOGO.cyan().bold());
}

#[tokio::main]
async fn main() {
    let raw_args: Vec<String> = env::args().collect();

    if raw_args.len() == 1 {
        print_logo();
        let _ = Cli::command().print_help();
        println!("\n");
        return;
    }

    let args = Cli::parse();
    let client = core::request::build_client();

    let mut headers_map: HashMap<String, String> = HashMap::new();
    for h in args.headers {
        if let Some((k, v)) = h.split_once(':') {
            headers_map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let auth_value = args.auth_value.clone().or(args.bearer.clone());
    core::authorization::auth(&mut headers_map, args.auth.clone(), auth_value);

    let mut queries_map: HashMap<String, String> = HashMap::new();
    for q in args.queries {
        if let Some((k, v)) = q.split_once('=') {
            queries_map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let rb = core::request::create_request(
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

            let spinner = assets::loading::create_spinner("Mengirim request...");
            let start = Instant::now();

            match req.send().await {
                Ok(res) => {
                    spinner.finish_with_message("Request selesai");
                    let duration = start.elapsed();

                    if let Err(e) = core::response::print_response(res).await {
                        eprintln!("{} {}", "Error printing response:".red(), e);
                    }

                    println!("\n{} {:?}", "⚡ Time Elapsed:".bold().blue(), duration);
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    assets::invalid::print_error("no_connection");
                    eprintln!("{} {}", "Request failed:".red(), e);
                }
            }
        }
        Err(e) => {
            if e.contains("URL tidak valid") {
                assets::invalid::print_error("url");
            } else if e.contains("Body bukan JSON valid") {
                assets::invalid::print_error("body");
            } else if e.contains("nggak dikenal") {
                assets::invalid::print_error("header");
            } else {
                assets::invalid::print_error("unknown");
            }
            eprintln!("{} {}", "Configuration error:".red(), e);
        }
    }
}
