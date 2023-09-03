use std::string::String;

struct Point<T, U> {
    first: T,
    second: U,
}

impl Point<i32, String> {
    fn new (first: i32, second: String) -> Point<i32, String> {
        Point {first, second}
    }

    fn get_first(&self) -> &i32{
        &self.first
    }
}

pub fn run() {
    let point = Point::new(123, String::from("test"));
    println!("first: {:?}, second: {:?}", point.first, point.second);

    let x = point.get_first();
    println!("first: {:?}", x);
}
