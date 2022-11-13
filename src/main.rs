use frec_rust::{matcher::Regex};


fn main() {
    let pattern = "Long Fox?";
    let text = "The Long Fox jumps over the lazy fence";

    let matcher = Regex::new(pattern).unwrap();
    println!("{:?}", matcher.find(text));
}
