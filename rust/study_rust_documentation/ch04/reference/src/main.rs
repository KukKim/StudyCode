// fn main() {
//     let s1 = String::from("hello");
//     let len = carculate_length(&s1);
//     println!("s1 - {}\nlen - {}", s1, len);
// }

// fn carculate_length(s:&String) -> usize {
//     s.push_str(", world!");
//     s.len()
// }

fn main() {
    let mut s1 = String::from("hello");
    let len = carculate_length(&mut s1);
    println!("s1 - {}\nlen - {}", s1, len);
    
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("r1 - {}\nr2 - {}", r1, r2);
    // Mutable reference cannot use with the other reference -> makes prevent data races
    // -data races*

    // death reference
    let reference_to_nothing = dangle();
}

fn carculate_length(s:&mut String) -> usize {
    s.push_str(", world!");
    s.len()
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}