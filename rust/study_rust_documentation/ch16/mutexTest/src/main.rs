use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

// * RefCell<T>/Rc<T> => Mutex<T>/Arc<T>

fn main() {
    // let m = Mutex::new(5);

    // println!("m = {:?}", m);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m);

    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result : {}", *counter.lock().unwrap());
}
