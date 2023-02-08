use rcTest::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5,
                    Rc::new(Cons(10,
                        Rc::new(Nil)))));

    println!("a's count - {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("a's count after b - {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("a's count after c - {}", Rc::strong_count(&a));
    }
    println!("a's count after c finished - {}", Rc::strong_count(&a));

}
