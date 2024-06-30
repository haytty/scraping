use clap::{Parser};
use crate::service;

#[derive(Parser, Debug, Clone)]
pub struct ApplyArgs {
    #[arg(long, default_value = "articles.csv")]
    dest: String,
}

pub async fn execute(args: ApplyArgs) -> Result<(), Box<dyn std::error::Error>> {
    service::generate_articles_csv::call(&args.dest).await
}