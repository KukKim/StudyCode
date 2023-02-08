pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read {}'s Article.",  self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub fn notify(item: impl Summary) {
    println!("Break news! : {}", item.summarize())
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebook"),
        content: String::from("Studying rust..."),
        reply: false,
        retweet:false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("hourse_ebook"),
        content: String::from("Studying rust..."),
        reply: false,
        retweet: false,
    };

    println!("new tweet: {}", tweet.summarize());
}
