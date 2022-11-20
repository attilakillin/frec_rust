use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Disregard multiline matches. Every match occurs on just one single line
    #[arg(short = 'l', long = "line", default_value_t = false)]
    pub singleline: bool,

    /// The pattern(s) to search for. To use multiple patterns, include the flag multiple times
    #[arg(short = 'p', long = "pattern")]
    pub patterns: Vec<String>,

    /// The file to search the pattern(s) in
    pub file: String
}
