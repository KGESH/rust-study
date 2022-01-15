use rust_study::Config;

use std::env;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap();

    if let Err(e) = rust_study::run(config) {
        println!("App error: {}", e);
        process::exit(1);
    }

}
