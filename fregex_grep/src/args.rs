use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The pattern(s) to search for. To use multiple patterns, include the flag multiple times
    #[arg(short = 'p', long = "pattern")]
    pub patterns: Vec<String>,

    /// The file to search the pattern(s) in
    pub file: String
}
