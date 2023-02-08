use box_ex01::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(2, Cons(3, Nil))))

    let list = Cons(1,
                    Box::new(Cons(2,
                        Box::new(Cons(3,
                            Box::new(Nil))))));
}
