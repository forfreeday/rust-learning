use std::{sync::{Mutex, Arc}, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut mutex_guard = counter.lock().unwrap();
            *mutex_guard += 1;
          
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
