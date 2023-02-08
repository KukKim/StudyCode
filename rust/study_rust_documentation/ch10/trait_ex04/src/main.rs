use std::fmt::Display;

struct Pair<T> {
    x:T,
    y:T,
}

impl<T> Pair<T> {
    fn new (x:T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest member x : {}", self.x);
        }else{
            println!("Largest member x : {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
