use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        // let val = String::from("Hello");
        // tx.send(val).unwrap();
        // // println!("val = {}", val);
        
        let vals = vec![
            String::from("Say"),
            String::from("Hello"),
            String::from("from"),
            String::from("child thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    thread::spawn(move || {
        // let val = String::from("Hello");
        // tx.send(val).unwrap();
        // // println!("val = {}", val);
        
        let vals = vec![
            String::from("Say again"),
            String::from("Hello again"),
            String::from("from again"),
            String::from("child thread again"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    // let received = rx.recv().unwrap();
    // println!("received: {}", received);

    for received in rx {
        println!("received: {}", received);
    };
}
