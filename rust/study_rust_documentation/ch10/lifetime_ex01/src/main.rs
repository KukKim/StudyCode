fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    // return's lifetime must have same lifetime with at least one parameter's lifetime
    // example> (not compiled)
    // fn longest<'a>(x: &str, y:&str) -> &'a str {...}
    
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("longer String: {}", result);

    let string1 = String::from("the longest String");
    // let result;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // result = longest(string1.as_str(), string2.as_str());
        println!("Loger String: {}", result);
    }
    // println!("Loger String: {}", result);
}
