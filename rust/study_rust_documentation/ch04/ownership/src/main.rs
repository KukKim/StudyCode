fn main() {
    let mut s = String::from("hello");
    println!("s - {}", s);
    s.push_str(", world!");
    println!("s - {}", s);

    let s1 = String::from("hello");
    println!("s1 - {}", s1);

    // let s2 = s1;
    // println!("s1 - {}", s1);
    let s2 = s1.clone();
    println!("s1 - {}", s1);
    println!("s2 - {}", s2);

    let s = String::from("hello");
    println!("s - {}", s);
    takes_ownership(s);
    // println!("s - {}", s);
    let x = 5;
    println!("x - {}", x);
    makes_copy(x);
    println!("x - {}", x);

    let s1 = gives_ownership();
    println!("s1 - {}", s1);
    let s2 = String::from("hello");
    println!("s2 - {}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("s3 - {}", s3);

    println!("s1 - {}", s1);
    // println!("s2 - {}", s2);
    println!("s3 - {}", s3);
}

fn takes_ownership(some_string:String){
    println!("String - {}", some_string);
}

fn makes_copy(some_interger:i32){
    println!("Integer - {}", some_interger);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}