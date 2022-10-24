use frec_rust::{matcher::Regex, flags::CompileFlags};


fn main() {
    let pattern = "Long Fox";
    let text = "The Long Fox jumps over the lazy fence";

    let flags = CompileFlags {
        use_extended: false,
    };

    let matcher = Regex::new(pattern, flags).ok().unwrap();
    println!("{:?}", matcher.find(text));
}
