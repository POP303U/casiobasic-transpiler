use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let contents = filereader(args[1].to_string());
    parse(contents)
}

fn filereader(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn parse(content: String) {
    let path = Path::new("output.txt");
    let display = path.display();
    let content: Vec<&str> = content.lines().collect();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(b"Hello, World!") {
        Err(why) => panic!("couldn't write to: {}: {}", display, why),
        Ok(file) => file,
    }
}
