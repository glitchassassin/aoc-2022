use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);
    let path = Path::new(filename.expect("No filename provided"));

    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open file: {}", why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file: {}", why),
        Ok(_) => println!("File loaded")
    };
    s
}