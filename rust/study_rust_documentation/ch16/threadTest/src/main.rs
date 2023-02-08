use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("new thread : {}", i);
    //         thread::sleep(Duration::from_millis(i));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("main thread : {}", i);
    //     thread::sleep(Duration::from_millis(i));
    // };

    // // handle.join().unwrap();


    let v = vec![1,2,3];
    
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}
