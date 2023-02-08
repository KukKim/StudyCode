pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value is not available(1<=value<=100) : {}", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) ->i32 {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
}
