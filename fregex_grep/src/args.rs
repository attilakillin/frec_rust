use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Use the library-supplied matcher instead of the custom one
    #[arg(short = 'o', long = "original", default_value_t = false)]
    pub original: bool,

    /// Return as soon as the first match is found
    #[arg(short = 'f', long = "first", default_value_t = false)]
    pub first_only: bool,

    /// The pattern(s) to search for. To use multiple patterns, include the flag multiple times
    #[arg(short = 'p', long = "pattern")]
    pub patterns: Vec<String>,

    /// The file to search the pattern(s) in
    pub file: String
}
