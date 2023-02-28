
fn main() {
    test1_push_str();
    test2_push();
    test3_plus();
}

fn test1_push_str() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn test2_push() {
    let mut s = String::from("lo");
    s.push('l');
    println!("s value: {}", s);
}

fn test3_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    // s1 失去所有权 
    // println!("s1 value: {}", s1);
    println!("s2 value: {}", s2);
    println!("s3 value: {}", s3);
}

fn test1() {
    let s1 = String::from("hello");
    let h = &s1[0];
}
