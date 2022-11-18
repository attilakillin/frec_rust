use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Use multi-pattern matching mode
    #[arg(short = 'm', long = "multi", default_value_t = false)]
    pub multi: bool,

    /// The pattern(s) to search for. Separate the patterns by newlines
    pub patterns: String,

    /// The file to search the pattern(s) in
    pub file: String
}
