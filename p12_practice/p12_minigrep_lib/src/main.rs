use std::process;
use std::env;
use p12_minigrep_lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = p12_minigrep_lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
