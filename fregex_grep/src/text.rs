use std::{fs::File, io::{Read, BufReader, BufRead, Lines}, process::exit};

pub fn read_text(path: &str) -> String {
    if let Ok(mut file) = File::open(path) {
        let mut string = String::new();

        match file.read_to_string(&mut string) {
            Ok(_) => return string,
            Err(_) => { println!("Error reading input file!"); exit(1); }
        }
    }

    println!("Error opening input file!"); exit(1);
}

pub fn read_text_lines(path: &str) -> Lines<BufReader<File>> {
    if let Ok(file) = File::open(path) {
        return BufReader::new(file).lines();
    }

    println!("Error opening input file!"); exit(1);
}
