use clap::Parser;
use fregex::{Regex, MultiRegex};
use std::fs;

use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();

    if args.multi {
        let patterns: Vec<&str> = args.patterns.split('\n').collect();
        let matcher = MultiRegex::new(&patterns);
    } else {
        let matcher = Regex::new(&args.patterns);
    }

    println!("{:?}", args);
}
