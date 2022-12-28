
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // 会报错：borrow of moved value: `s1`
    // 因为所有权已经转移
    // println!("{}, world!", s1);

    println!("{}, world!", s2);
    test_clone();
}

fn test_clone() {
    println!("test_clone");
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
