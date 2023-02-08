fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);


    // let v = vec![1, 2, 3, 4, 5];
    // // v.push(6);
    // let third: &i32 = &v[2];
    // println!("third - {}", third);

    // match v.get(2) {
    //     Some(third) => println!("third - {}", third),
    //     None => println!("no third"),
    // }

    // let v = vec![1, 2, 3, 4, 5];
    // for i in &v {
    //     println!("{}", i)
    // }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 100;
    }
    for i in &v {
        println!("{}", i)
    }
}
