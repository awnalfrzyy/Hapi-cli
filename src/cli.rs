use clap::Parser;

#[derive(Parser)]
#[command(
    name = "awin-req",
    version = "1.0",
    about = "Simple & Fast HTTP Client"
)]
pub struct Cli {
    pub method: String,
    pub url: String,

    #[arg(short, long)]
    pub body: Option<String>,

    #[arg(short = 'H', long)]
    pub headers: Vec<String>,

    #[arg(short = 'q', long)]
    pub queries: Vec<String>,
}
