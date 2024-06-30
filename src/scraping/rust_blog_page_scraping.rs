use scraper::ElementRef;
use url::Url;
use crate::model::article::Article;

const RUST_BLOG_PAGE_URL: &str = "https://blog.rust-lang.org/";

fn get_published_at(td_elements: &Vec<ElementRef>) -> Option<String> {
    let published_at_td_element = td_elements.get(0)?;
    let published_at_text = published_at_td_element.text().next()?;

    let normalized_published_string = published_at_text.replace("\u{a0}", " ");
    Some(normalized_published_string)
}

fn get_title(td_elements: &Vec<ElementRef>) -> Option<String> {
    let article_title_td_element = td_elements.get(1)?;
    let article_title_a_elements: Vec<_> = article_title_td_element.child_elements().collect();
    let article_title_a_element = article_title_a_elements.get(0)?;

    let article_title_str = article_title_a_element.text().next()?;

    Some(article_title_str.to_string())
}

fn get_url_path(td_elements: &Vec<ElementRef>) -> Option<String> {
    let article_title_td_element = td_elements.get(1)?;
    let article_title_a_elements: Vec<_> = article_title_td_element.child_elements().collect();
    let article_title_a_element = article_title_a_elements.get(0)?;

    let url_path_str = article_title_a_element.attr("href")?;

    Some(url_path_str.to_string())
}

fn html_body_to_article(body: String) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);

    let tr_selector = scraper::Selector::parse("tbody > tr")?;
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
