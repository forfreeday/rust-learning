fn main() {
    let result = get_string();
    println!("Result: {}", result);

    let another_result = result; // 所有权转移

    // 编译错误：`result` 超出了其作用域
    // println!("Result: {}", result);

    println!("Another Result: {}", another_result);
}

fn get_string() -> String {
    String::from("Hello")
}
