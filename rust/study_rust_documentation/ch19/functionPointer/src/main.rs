fn add_one (x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("{}", answer);

    let list_of_numbers = vec![1,2,3];
    // let list_of_stirngs : Vec<String> = list_of_numbers
    //     .iter()
    //     .map(|i| i.to_string())
    //     .collect();

    let list_of_stirngs: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_stirngs);
}


// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}