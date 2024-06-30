mod cli;
mod model;
mod service;
mod scraping;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::start().await
}
