struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    // ChangeColor(i32, i32, i32),
    ChangeColor(Color)
}

enum Message2 {
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

fn main() {
    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("50"),
    //     Some(y) => println!("Same, y = {:?}", y),
    //     _ => println!("Not Same, x = {:?}", x),
    // }

    // println!("result: x= {:?}, y= {:?}", x, y);

    // let x = 1;

    // match x {
    //     1|2 => println!("1 or 2"),
    //     3 => println!("3"),
    //     _ => println!("Something else")
    // }
    // let x = 3;
    // match x {
    //     1 ... 5 => println!("1 ~ 5"),
    //     _ => println!("Something else")
    // }
    // let x = 'd';
    // match x {
    //     'a'...'f' => println!("a ~ f"),
    //     _ => println!("Something else")
    // }




    // let p = Point{x: 0, y: 7};

    // // let Point {x: a, y: b} = p;
    // // assert_eq!(0, a);
    // // assert_eq!(7, b);

    // let Point {x, y} = p;
    // assert_eq!(0, x);
    // assert_eq!(7, y);

    // match p {
    //     Point {x, y:0} => println!("Point is on the x axis at {}", x),
    //     Point {x:0, y} => println!("Point is on the y axis at {}", y),
    //     Point {x, y} => println!("Point is ({}, {})", x, y),
    // }

    // // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    // match msg {
    //     Message::Quit => {
    //         println!("Quit: No dissolve value")
    //     },
    //     Message::Move { x, y } => {
    //         println!("Move: x = {}, y = {}", x, y);
    //     },
    //     Message::Write(text) => println!("Write: {}", text),
    //     // Message::ChangeColor(r, g, b) => {
    //     //     println!("ChangeColor: R = {}, G = {}, B = {}", r, g, b);
    //     // }
    //     Message::ChangeColor(Color::Rgb(r, g, b)) => {
    //         println!("ChangeColor : R = {}, G= {}, B = {}", r, g, b);
    //     },
    //     Message::ChangeColor(Color::Hsv(h, s, v)) => {
    //         println!("ChangeColor : H = {}, S= {}, V = {}", h, s, v);
    //     },
    //     _ => {}
    // }

    // let ((feet, inches), Point{x, y}) = ((3, 10), Point {x:3, y:-10});

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match(setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cannot overwrite");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting value : {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("numbers : {}, {}, {}", first, third, fifth);
        }
    }

    let _x = 5; // _ makes variable doesn't cause warning message. Although variable not used.
    let y = 10; // cause warining message. Because variable does not used.

    let s = Some(String::from("Hello"));
    // if let Some(_s) = s { // => _s( _+"string" variable ) binds value.
    if let Some(_) = s{ // But, _ variable doesn't bind value.
        println!("Literal found.");
    }
    println!("{:?}", s);

    let origin = Point2 {x:0, y:0, z:0};

    match origin {
        Point2 { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("first = {}, last = {}", first, last);
        }
    }
    // match numbers { //Compiler cannot understand this pattern.
    //     (.., second, ..) => {
    //         println!("second = {}", second);
    //     }
    // }

    // Match guard
    let num = Some(4);
    match num {
        Some(x) if x<5 => println!("samaller than 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("50"),
        Some(n) if n == y => println!("Same, n = {:?}", n),
        _ => println!("Not same, x = {:?}", x),
    }

    println!("Result : x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }

    // @
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3...7 } => {
            println!("Id is found in range. : {}", id_variable)
        },
        Message2::Hello { id: 10...12 } => {
            println!("id is found other range.")
        },
        Message2::Hello { id } => {
            println!("other id {} found.", id)
        },
    }
}

fn foo(_: i32, y: i32) {
    println!("This Function use parameter y only. :{}", y);
}