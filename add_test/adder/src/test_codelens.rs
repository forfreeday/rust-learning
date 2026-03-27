// 这是一个简单的测试文件，用于验证 CodeLens 功能

fn main() {
    println!("如果你能看到这个 main 函数上方的 Run 和 Debug 按钮，说明 CodeLens 正常工作");
}

#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
