use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let filename = args[1].to_string();
    filereader(filename);
}

fn filereader(filename: String) {
    println!("{}", filename);
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("Content:\n{}", contents)
}
