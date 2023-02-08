fn main() {
    // let len1 = String::from("Hello").len();
    // println!("{}", len1);

    // let len2 = String::from("안녕하세요").len();
    // println!("{}", len2);

    let hello = "안녕하세요";
    // let s = &hello[0..3];
    // println!("{}", s)

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}
