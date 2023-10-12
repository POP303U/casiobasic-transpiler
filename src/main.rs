use std::fs::File;
use std::io::{prelude::*, Read};
use std::path::Path;
use std::{env, result};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let contents = filereader(args[1].to_string());
    let _result = parse(contents);
}

fn filereader(filename: String) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn parse(content: String) -> std::io::Result<()> {
    let path = Path::new("OUTPUT.txt");
    let display = path.display();
    let lines: Vec<&str> = content.lines().collect();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write_all(String::from("'ProgramMode:RUN\n").as_bytes())?;
    for line in lines {
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        if tokens.len() == 4 && tokens[0] == "local" && tokens[2] == "=" {
            let var_name = tokens[1].to_ascii_uppercase();
            let var_value = tokens[3];
            let final_string = var_value.to_owned() + "->" + &var_name + "\n";
            file.write_all(final_string.as_bytes())?;
        }
    }
    Ok(())
}
