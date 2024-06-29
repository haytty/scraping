pub mod rust_blog_page;

pub trait Page {
    fn target(&self) -> String;
}