use colored::*;
use reqwest::Response;

pub async fn print_response(res: Response) -> Result<(), Box<dyn std::error::Error>> {
    let status = res.status();
    let status_color = if status.is_success() {
        status.to_string().green()
    } else {
        status.to_string().red()
    };

    println!("\n{} {}", "Status:".bold(), status_color);

    let content_type = res
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let body = res.text().await?;

    if content_type.contains("application/json") {
        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(json) => {
                println!("{}", "--- Response Body (JSON) ---".cyan());
                println!("{}", serde_json::to_string_pretty(&json)?);
            }
            Err(_) => {
                println!("{}", "--- Response Body (Invalid JSON) ---".yellow());
                println!("{}", body);
            }
        }
    } else {
        println!("{}", "--- Response Body (Raw) ---".magenta());
        println!("{}", body);
    }

    Ok(())
}
