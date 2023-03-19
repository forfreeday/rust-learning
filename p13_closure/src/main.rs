use std::{thread, time::Duration, env};

struct Cacher<T>
    where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T> 
    where T: Fn(u32) -> u32, {
   fn new (calculation: T) -> Cacher<T> {
       Cacher { calculation, value: None, }
   } 

   fn value(&mut self, arg: u32) -> u32 {
      match self.value {
          Some(v) => v,
          None => {
              let v = (self.calculation)(arg);
              self.value = Some(v);
              v
          }
      } 
   }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculation slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let intensity = &args[1];

    if intensity.len() < 25 {
        println!("result: {}", expensive_closure.value(24));
    }

}
