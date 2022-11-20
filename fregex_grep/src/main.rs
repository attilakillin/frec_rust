use clap::Parser;
use fregex::{Regex, MultiRegex, RegexMatcher};
use text::read_text;
use std::process::exit;

use crate::args::Args;

mod args;
mod text;

fn main() {
    // Parse arguments.
    let args = Args::parse();
    if args.patterns.len() == 0 {
        println!("No patterns were supplied!"); exit(1);
    }
    let patterns: Vec<&str> = args.patterns.iter().map(|p| p.as_str()).collect();

    // Create proper matcher.
    let matcher: Box<dyn RegexMatcher> = if args.patterns.len() > 1 {
        match MultiRegex::new(&patterns) {
            Ok(matcher) => Box::new(matcher),
            Err(_) => { println!("Error parsing patterns!"); exit(1); }
        }
    } else {
        match Regex::new(patterns[0]) {
            Ok(matcher) => Box::new(matcher),
            Err(_) => { println!("Error parsing pattern!"); exit(1); }
        }
    };

    // Read text and find every single match.
    let string = read_text(&args.file);
    let mut text: &str = &string;
    let mut offset = 0;

    while text.len() > 0 {
        if let Some(result) = matcher.find(&text) {
            let start = result.start() + offset;
            let end = result.end() + offset;

            text = &text[result.end()..];
            offset += result.end();

            println!("({}, {})", start, end);
        } else {
            break;
        }
    }
}
