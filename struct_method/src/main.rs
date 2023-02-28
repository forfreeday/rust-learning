
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 注意这里是借用，使用的 &，也获得所有权，或者是一个可变的'借用'
    fn area(&self) -> u32 {
        self.width * self.height
    } 
}

fn main() {
    let rect1 = Rectangle {
        width : 30,
        height : 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}
