use std::{fs::File, io::Read, process::exit};

use memmap::{MmapOptions, Mmap};

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

pub fn read_text_mmap(path: &str) -> Mmap {
    if let Ok(file) = File::open(path) {

        match unsafe { MmapOptions::new().map(&file) } {
            Ok(map) => return map,
            Err(_) => { println!("Error reading input file!"); exit(1); }
        }
    }

    println!("Error opening input file!"); exit(1);
}
