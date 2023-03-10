use oop_ex03_pattern_state::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch today.", post.content());
}
