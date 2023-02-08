use std::collections::HashMap;

fn main() {

    // // 1. value change
    // let mut scores = HashMap::new();

    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("blue"), 25);

    // println!("{:?}", scores);

    // // 2. value change(when key doesn't have value)
    // let mut scores = HashMap::new();

    // scores.insert(String::from("blue"), 10);
    // scores.entry(String::from("yellow")).or_insert(50);
    // scores.entry(String::from("blue")).or_insert(50);

    // println!("{:?}", scores);

    // 3. conditional value change
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
