
#[derive(Debug)]
struct Rectangle {
    ridth: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        ridth: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}
