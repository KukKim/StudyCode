use std::slice;

static HELLO_WORLD : &str = "Hello!";
static mut COUNTER : u32 = 0;

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    println!("{:?}, {:?}", a, b);


    // let address = 0x012345usize;
    // let r = address as *mut i32;

    // let slice: &[i32] = unsafe {
    //     slice::from_raw_parts_mut(r, 10000)
    // };

    unsafe {
        println!("C : -3's abs is ... : {}", abs(-3));
    }

    println!("static : {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER : {}", COUNTER);
    }
}


// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid<=len);
    
//     (&mut slice[..mid], &mut slice[mid..])
// }


fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid<=len);
    
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C"{
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Call rust's function from C!");
}

fn add_to_count(inc: u32){
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}