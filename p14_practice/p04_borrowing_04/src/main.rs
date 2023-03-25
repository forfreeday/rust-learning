// test4
// 所有权 发生 移动
// s1 移动到 s2
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
}

