fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}",word);
    s.clear();
    // println!("{}",word);

    // let hello = &s[0..5]; // &s[..5]
    // let world = &s[6..11]; // &s[6..s.len()] == &s[6..]
    // let all = &s[..];
    // println!("hello - {}\nworld - {}\nall - {}", hello, world, all);

}

// fn first_word(s:&String) -> usize {
//     let bytes = s.as_bytes();

//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s:&str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}