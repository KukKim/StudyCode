
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};


fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("first a rc count = {}", Rc::strong_count(&a));
    println!("a's next = {:?}", a.tail());

    let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b created = {}", Rc::strong_count(&a));
    println!("first b rc count = {}", Rc::strong_count(&b));
    println!("b's next = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after a edited = {}", Rc::strong_count(&b));
    println!("a rc count after a edited = {}", Rc::strong_count(&a));


    // *cause overflow!
    // println!("a's next = {:?}", a.tail());
}
