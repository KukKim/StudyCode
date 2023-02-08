use dropTest::CustomSmartPointer;

fn main() {
    let c = CustomSmartPointer { data: String::from("my data")} ;
    let d = CustomSmartPointer { data: String::from("other data") };
    // c.drop();  // cannot process drop method
    drop(c);

    println!("CustomSmartPointer Created.")
}
