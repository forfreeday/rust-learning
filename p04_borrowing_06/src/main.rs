
fn main() {
    let a = String::from("Hello");
    let b = test(a);
    println!("in main, a value: {}", b);
}

fn test(a: String) -> String{
    println!("a value is :{}", &a);
    a
}
