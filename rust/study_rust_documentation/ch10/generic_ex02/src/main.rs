struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Point {x:5, y:10};
    let float = Point {x:1.0, y:4.0};

    let mix = Point2 {x:3, y:1.2};

    let p = Point{ x: 5, y: 4 };
    println!("p.x = {}", p.x());
}
