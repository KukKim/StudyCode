use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     // Err(error) => {
    //     //     panic!("File open failed: {:?}", error)
    //     // }
    //     Err(ref error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Can't create file: {:?}", e)
    //         },
    //         other_error => panic!("Can't open file: {:?}", other_error)
    //     }
    // };

    let f = File::open("hello1.txt").unwrap();
    // let f = File::open("hello1.txt").expect("Can't open file.");
}
