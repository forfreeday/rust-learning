use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_file(filename: &str) -> Result<(), io::Error> {
    let contents = read_file_contents(filename)?;
    // 执行其他操作...
    Ok(())
}

fn main() {
    match process_file("example.txt") {
        Ok(()) => println!("File processed successfully"),
        Err(error) => println!("Error processing file: {}", error),
    }
}
