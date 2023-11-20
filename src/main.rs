use std::{fs::File, io::{BufRead, BufReader}};
use clap::{Parser, Arg};
use jsonwebtoken::{decode, Algorithm, Validation};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer - a simple CLI to transform and inspect strings", long_about = "stringer is a super fancy CLI (kidding)")]

#[derive(Subcommand)]
enum Commands {
    /// Reverses a string
    Info(Info),
    /// Inspects a string
    Crack(Crack),
}

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct Info {
    /// The string to info
    string: Option<String>,
}

#[derive(Args)]
struct Crack {
    /// The string to crack
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}


fn main() {
    let cli = Cli::parse();
}
