pub trait Summary {
    fn summerize(&self) -> String {
        String::from("read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline example"),
        location: String::from("location maybe somewhere"),
        author: String::from("wikiwiki!"),
        content: String::from("well... this is content."),
    };

    println!("new article : {}", article.summerize());
}
