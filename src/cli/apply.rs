use clap::{Parser};
use crate::service;

#[derive(Parser, Debug, Clone)]
pub struct ApplyArgs {}

pub async fn execute(_args: ApplyArgs) -> Result<(), Box<dyn std::error::Error>> {
    service::generate_articles_csv::call().await
}