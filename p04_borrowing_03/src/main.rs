
fn calculate_length(s: &String) -> usize {
    s.len()
}



fn test3() {
    let list = vec![1, 2, 3];
    {
        let list2 = vec![1, 2, 3];
    }
    println!("Before defining closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    println!("Before calling closure: {:?}", list2);
}

