use oop_ex04::{Post, DraftPost};

fn main() {
    let mut post = Post::new();
    
    post.add_text("Hello, Let's talk about OOP.");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("Hello, Let's talk about OOP.", post.content());
}
