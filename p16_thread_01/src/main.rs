use std::thread;


fn main() {

    thread::spawn(||{
        for i in 0..20 {
            println!("sub thread print: {}", i);
            
        }
    });

    for i in 0..10 {
       println!("main thread print: {}", i);
    }
}
