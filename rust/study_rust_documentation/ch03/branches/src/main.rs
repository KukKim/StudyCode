fn main() {
    let number = 9;

    if number > 5 {
        println!("number is higher than 5")
    }else if number == 5 {
        println!("number is 5")
    } else {
        println!("number is lower than 5")
    }

    let num_change = if number > 5 {
        println!("{} -> 1", number);
        1
    }else if number == 5 {
        println!("{} -> 0", number);
        0
        // "영"
    }else {
        println!("{} -> 11", number);
        -1
    };
}
