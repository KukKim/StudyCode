#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};


fn main() {
    // let x = 5;
    // let y= &mut x;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;

    println!("after edit a = {:?}", a);
    println!("after edit b = {:?}", b);
    println!("after edit c = {:?}", c);
}
