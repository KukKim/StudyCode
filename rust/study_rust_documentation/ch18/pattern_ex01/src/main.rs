

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Backgorund color is your favorite color : {}", color);
    } else if is_tuesday {
        println!("Green is the best choice for tuesday!")
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Purple is good for you.")
        } else {
            println!("How about Orange Color?")
        }
    } else {
        println!("Blue Background available")
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("Index {}'s value : {}", index, value);
    }

    let point = (3,5);
    print_coordinates(&point);


    // Refutable Pattern vs Irrefutatble Pattern
    // Refutable - if let, while let...
    // Irrefutable - function's parameter, let, for...

    let some_option_value = Some(5);
    // let Some(x) = some_option_value;
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("coordinate: ({}, {})", x, y);
}