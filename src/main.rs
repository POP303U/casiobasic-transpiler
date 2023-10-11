use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let contents = filereader(args[1].to_string());
    println!("Content:\n{}", contents);
}

fn filereader(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn parse(content: String) {}
