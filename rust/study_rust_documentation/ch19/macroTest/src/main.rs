
#[macro_export]
macro_rules! vec {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]

struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello macro! My name is Pancakes!");
//     }
// }

fn main() {
    let v: Vec<u32> = vec![1 ,2, 3];

    Pancakes::hello_macro();
}
