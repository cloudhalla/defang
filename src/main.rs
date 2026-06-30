use std::io::{self, BufRead};

use clap::Parser;

use defang::{defang, refang};

#[derive(Parser)]
#[command(
    name = "defang",
    version,
    about = "Defang (or refang) URLs, domains, IPv4/IPv6 addresses, and email addresses",
    long_about = "Makes indicators of compromise (IoCs) inert for safe sharing.\n\
                  Reads from positional arguments; falls back to stdin when none are given."
)]
struct Cli {
    /// Strings to process (reads from stdin line-by-line when omitted)
    #[arg(value_name = "INPUT")]
    inputs: Vec<String>,

    /// Refang previously defanged strings instead of defanging
    #[arg(short, long)]
    refang: bool,
}

fn main() {
    let cli = Cli::parse();

    let transform: fn(&str) -> String = if cli.refang { refang } else { defang };

    if cli.inputs.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => println!("{}", transform(&input)),
                Err(e) => eprintln!("error reading stdin: {e}"),
            }
        }
    } else {
        for input in &cli.inputs {
            println!("{}", transform(input));
        }
    }
}
