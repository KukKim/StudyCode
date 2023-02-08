// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest<T>(list: &[T]) -> T {
// fn largest<T: PartialOrd>(list: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // let number_list = vec![34, 25, 51, 82, 13];

    // let result = largest_i32(&number_list);
    // println!("Largest number : {}", result);


    // let char_list = vec!['z', 'e', 'v', 'x', 'a'];

    // let result = largest_char(&char_list);
    // println!("Largest char : {}", result);

    let number_list = vec![34, 25, 51, 82, 13];

    let result = largest(&number_list);
    println!("Largest number : {}", result);


    let char_list = vec!['z', 'e', 'v', 'x', 'a'];

    let result = largest(&char_list);
    println!("Largest char : {}", result);
}
