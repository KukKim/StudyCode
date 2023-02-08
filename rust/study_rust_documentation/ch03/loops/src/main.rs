fn main() {
    // loop example
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter *2;
    //     }else{
    //         println!("counter - {}", counter)
    //     }
    // };
    // println!("result - {}", result)

    // while example
    // let mut number = 3;
    // while number != 0 {
    //     println!("number - {}", number);
    //     number -= 1;
    // }
    // println!("done!")

    // for example
    let a = [1,2,3,4,5];
    // let mut index = 0;
    // while index < 5 {
    //     println!("a[{}] - {}", index, a[index]);
    //     index = index + 1;
    // }
    for element in a.iter() {
        println!("element - {}", element)
    }
    for number in (1..4).rev() {
        println!("{}!", number)
    }
}
