fn main() {
    println!("Hello, world!");
    another_function(5, 3.2);

    // let x = (let y = 6);
    let y = {
        let x = 3;
        x + 1 // 표현식은 마지막에 (;)을 포함하지 않음
    };
    println!("{}",y);
    let x = five();
    println!("{}", x);
    let y = add_one(x);
    println!("{}", y);
}

fn another_function(x: i32, y: f32) {
    println!("Another function! \n - {}\n - {}", x, y);
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    // return x+1;
    x+1
}