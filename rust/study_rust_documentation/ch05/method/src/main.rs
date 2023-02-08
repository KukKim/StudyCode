#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    } 
}

fn main() {
    let rect1 = Rect { width: 30, height: 50 };

    println!(
        "{}",
        rect1.area()
    );

    let rect2 = Rect { width: 50, height: 40 };

    println!(
        "{}",
        rect2.area()
    );

    let rect3 = Rect { width: 50, height: 90 };

    println!(
        "{}",
        rect3.area()
    );

    println!{
        "rect2 include rect1 : {}",
        rect2.can_hold(&rect1)
    };
    println!{
        "rect3 include rect1 : {}",
        rect3.can_hold(&rect1)
    };

    Rect::square(3);
}
