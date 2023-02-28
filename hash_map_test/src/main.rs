use std::collections::HashMap;

fn main() {
    test_insert();
    test_get();
    test_split();
}


fn test_insert() {
    let field_key = String::from("test_key");
    let field_value = String::from("test_value");
    let mut map = HashMap::new();
    //map.insert(field_key, field_value);
    map.insert(&field_key, &field_value);
    println!("key: {}, value: {}", field_key, field_value);
    
}

fn test_get() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}", score);
}

fn test_split() {
    let text = "hello world wonderful world";
    
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
