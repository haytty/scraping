use std::fs::File;
use crate::scraping;

pub async fn call(dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let articles = scraping::rust_blog_page_scraping::call().await?;

    let f = File::create(dest)?;

    let mut writer = csv::Writer::from_writer(f);
    for e in articles.into_iter() {
        writer.serialize(e)?;
    }

    Ok(())
}
