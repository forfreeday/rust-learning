use std::fs::File;

fn main() {
    println!("Hello, world!");
    panic!("ir here!");
    let v = vec![1, 2, 3];
    v[99];
    panic_test1();
}

fn panic_test1() {
    let f = File::open("test.txt");
    let _f = match f {
        Ok(file)=>file,
        Err(error)=>panic!("Problem opening the file: {:?}", error),
    };

    println!("Hello, world!");
}
