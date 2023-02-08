struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention! {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("This is novel. But I don't have story.");
    let first_sentence = novel.split('.')
        .next()
        .expect("Can't find sentence divider.");
    let i = ImportantExcerpt { part: first_sentence };

}
