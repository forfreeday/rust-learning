fn main() {
    returns_clousure();
}

// 反例
// fn returns_clousure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }


fn returns_clousure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
