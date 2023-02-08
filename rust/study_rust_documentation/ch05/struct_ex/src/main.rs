#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}
fn main() {
    // let width = 30;
    // let height = 50;
    // println!("{}", area1(width, height));
    
    // let rect = (30, 50);
    // println!("{}", area2(rect));

    let rect = Rect { width: 30, height: 50 };
    println!("{:#?}", &rect);
    println!("{}", area3(&rect));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rect) -> u32 {
    rect.width * rect.height
}