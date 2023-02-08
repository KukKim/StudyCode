#[derive(Debug)]
enum UsState {
    Alabama, Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents (coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// match must cover all case!
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some( i + 1 ),
    }
}

fn main() {
    let alibama_quarter = value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let coin = Coin::Penny;
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("{:?}주의 25센트 동전!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("{:?}주의 25센트 동전!", state);
    }else {
        count += 1;
    }
}
