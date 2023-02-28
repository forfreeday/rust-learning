
fn main() {
    let mut v = Vec::new();
    v.push(1);
    test1();
    test2_match();
    test3_out_bounds();
    test4_each();
    test5_each();
}

fn test1() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let value = &v[2];
    print!("value: {}\n", value)
}

fn test2_match() {
    let mut v = Vec::new();  
    v.push(1);
    v.push(2);
    v.push(3);

    match v.get(100) {
        Some(third)=> println!("value: {}", third), 
        None=>println!("None"),
    };
}

fn test3_out_bounds() {
    // let v = vec![1, 2, 3, 4, 5];
    //
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
}

fn test4_each() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    } 
}

fn test5_each() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for j in &mut v {
       println!("result: {}", j) 
    }

}
