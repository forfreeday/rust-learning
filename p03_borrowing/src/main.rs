// test1
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// test2
fn test2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
}

fn test2_1() {
    let list = vec![1, 2, 3];
    {
        let list2 = vec![1, 2, 3];
    }
    println!("Before defining closure: {:?}", list);
    println!("Before calling closure: {:?}", list);

    println!("Before calling closure: {:?}", list2);
}

// test3
fn test3() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
}

