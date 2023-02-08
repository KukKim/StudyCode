pub struct MyBox<T>(T);
use std::ops::Deref;

impl<T> MyBox<T> {
    pub fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn hello(name: &str) {
    println!("Hello {}!", name);
}