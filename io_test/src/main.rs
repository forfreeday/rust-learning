use std::fs::File;
use std::io::{self, BufRead};

// 读取文件
fn main() -> io::Result<()> {
    test1()
}

fn test1() -> io::Result<()> {
    let file = File::open("/Users/liukai/Downloads/1.txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
