use std::env;
use std::fs::File;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("command {}", query);
    println!("file name {}", filename);

    let mut file = File::open(filename)
        .expect("File not found!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading to string!");

    println!("With Text : \n{}", contents);

}
