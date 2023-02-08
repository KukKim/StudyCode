fn main() {
    // let mut s = String::new();

    // let data = "string initialize1";

    // let s1 = data.to_string();
    // let s2 = "string initialize2".to_string();
    // let s3 = String::from("string initialize3");

    // println!("{}\n{}\n{}", s1, s2, s3)

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("{}", s);

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("{}", s1);
    // println!("{}", s2);
    // let mut s3 = String::from("lo");
    // s3.push('l');
    // println!("{}", s3);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);
}
