fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("largest number - {}", largest);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("largest number : {}", result);


    let number_list = vec![4223, 25511, 4214, 23123, 2341];

    let result = largest(&number_list);
    println!("largest number : {}", result);
}
