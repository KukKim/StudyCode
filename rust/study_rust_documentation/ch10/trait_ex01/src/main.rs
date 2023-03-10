pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content )
    }
}

pub trait Summary {
    fn summerize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Studying rust"),
        reply: false,
        retweet: false,
    };

    println!("new tweet : {}", tweet.summerize());
}
