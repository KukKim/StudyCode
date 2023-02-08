use derefTest::MyBox;
use derefTest::hello;

fn main() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    hello(&m);
}
