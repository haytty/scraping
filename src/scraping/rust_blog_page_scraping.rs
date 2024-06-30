use scraper::ElementRef;
use url::Url;
use crate::model::article::Article;

const RUST_BLOG_PAGE_URL: &str = "https://blog.rust-lang.org/";

fn get_published_at(td_elements: &Vec<ElementRef>) -> Option<String> {
    let s = td_elements.get(0)?.text().next()?.replace("\u{a0}", " ");
    Some(s)
}

fn get_title(td_elements: &Vec<ElementRef>) -> Option<String> {
    let a_elements: Vec<_> = td_elements.get(1)?.child_elements().collect();
    let a_element = a_elements.get(0).unwrap();
    let s = a_element.text().next()?.to_string();

    Some(s)
}

fn get_url_path(td_elements: &Vec<ElementRef>) -> Option<String> {
    let a_elements: Vec<_> = td_elements.get(1)?.child_elements().collect();
    let a_element = a_elements.get(0)?;
    let url_str = a_element.attr("href")?;

    Some(String::from(url_str))
}

fn html_body_to_article(body: String) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let tr_selector = scraper::Selector::parse("tbody > tr").unwrap();

    let tr_elements = document.select(&tr_selector);
    let base_url = Url::parse(RUST_BLOG_PAGE_URL)?;

    let articles: Vec<Option<Article>> = tr_elements.map(|tr_element| {
        let td_elements: Vec<_> = tr_element.child_elements().collect();

        let published_at = get_published_at(&td_elements)?;
        let title = get_title(&td_elements)?;
        let url_path = get_url_path(&td_elements)?;
        let url = base_url.join(&url_path).unwrap();

        Some(Article::new(title, published_at, url.to_string()))
    }).collect();

    // unwrap
    let articles: Vec<Article> = articles.into_iter().filter_map(|x| x).collect();
    Ok(articles)
}

pub async fn call() -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let body = reqwest::get(RUST_BLOG_PAGE_URL)
        .await?
        .text()
        .await?;

    let articles = html_body_to_article(body)?;

    Ok(articles)
}
