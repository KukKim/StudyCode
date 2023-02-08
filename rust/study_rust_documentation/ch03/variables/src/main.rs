fn main() {
    let mut x = 5;
    println!("x = {}", x);
    x = 10;
    println!("x = {}", x);
    const Y:u32 = 3;
    println!("Y = {}", Y);

    //variable shadow
    let x = 1;
    println!("x = {}", x);
    let x = x*10;
    println!("x = {}", x);

    let spaces = "    ";
    println!("spaces = {}", spaces);
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // let mut mut_spaces = "    ";
    // println!("mut_spaces = {}", mut_spaces);
    // mut_spaces = mut_spaces.len();
    // println!("mut_spaces = {}", mut_spaces);
    
    // Data Types
    let guess: u32 = "42".parse().expect("Not number");
    // let guess2 = "42".parse().expect("Not number");
    // Scalar Types

    let x = 2.0;
    let y: f32 = 3.0;
    
    let sum = x+y;
    println!("sum = {}", sum);
    let difference = x-y;
    println!("difference = {}", difference);
    let product = x*y;
    println!("product = {}", product);
    let quitient = x/y;
    println!("quitient = {}", quitient);
    let remainder = x%y;
    println!("remainder = {}", remainder);

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}{}{}",c,z,heart_eyed_cat);
    
    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 5.3, 1);

    let (x,y,z) = tup;
    
    println!("y = {}", y);
    println!("1 = {}", tup.0);
    println!("2 = {}", tup.1);
    println!("3 = {}", tup.2);
    
    // Array Type
    let a = [1,2,3,4,5];
    // println!("{}", a.0);
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let aArray: [i32; 5] = [1,2,3,4,5];
    let aArray = [3; 5];
    let month_1 = months[0];
    println!("{}", month_1);
    let month_2 = months[1];
    println!("{}", month_2);
    let month_3 = months[2];
    println!("{}", month_3);
    // Out of bound
    let month_13 = months[12];
    println!("{}", month_13); 
}