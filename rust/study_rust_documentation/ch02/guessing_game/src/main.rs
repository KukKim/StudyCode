use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Let's start guessing game!");
    println!("Put a number you guess!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("random number is {}.", secret_num);

    loop{
        let mut guess = String :: new(); // mut -> make variable mutable
        io::stdin().read_line(&mut guess)
            .expect("number didn't read");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Not number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you put... {}", guess);

        match guess.cmp(&secret_num){
            Ordering::Less => println!("greater"),
            Ordering::Greater => println!("smaller"),
            Ordering::Equal => {
                println!("Good!!!");
                break;
            },
        }
    }
}