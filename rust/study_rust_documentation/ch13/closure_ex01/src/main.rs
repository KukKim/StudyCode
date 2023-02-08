use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Calculating...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }


// closure & function
// fn add_one_v1  (x: u32) -> u32 { x + 1 } 
// let add_one_v2 |x: u32| -> u32 { x + 1 }; 
// let add_one_v3 |x|             { x + 1 };
// let add_one_v4 |x|               x + 1;

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
    //     println!("Calculating...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("Calculating...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_closure = Cacher::new(|num: u32| -> u32 {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Push-ups {} times, first!!",
            expensive_closure.value(intensity)
        );
        println!(
            "Sit-up {} times, next!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take rest!");
        } else {
            println!(
                "Run {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );


    // closure's parameters & return's types must same!
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let s = example_closure(5);

}
