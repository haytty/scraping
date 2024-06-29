use clap::{Parser};
use crate::page::rust_blog_page::RustBlogPage;

#[derive(Parser, Debug, Clone)]
pub struct ApplyArgs {}

pub async fn execute(_args: ApplyArgs) -> Result<(), Box<dyn std::error::Error>> {
    let page = RustBlogPage::new();

    let articles = page.build_article().await?;

    // println!("{}", articles.iter().count());

    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for e in articles.into_iter() {
        writer.serialize(e)?;
    }

    Ok(())
}