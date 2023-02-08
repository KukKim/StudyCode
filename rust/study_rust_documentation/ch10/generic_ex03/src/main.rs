struct Point<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point{x: 5, y: 3.5};
    let p2 = Point{x: "Hella", y:'s'};

    let p3 = p1.mixup(p2);
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}

// Generic doesn't make runtime slower! Because Compiler make each types...
// let integer = Some(5);
// let float = Some(5.0);
// enum Option<T> {...} ---Compiler---> enum Option_i32 {...}; enum Option_i64 {...};