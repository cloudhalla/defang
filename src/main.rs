use std::io::{self, BufRead};

use clap::Parser;

use defang::{defang, refang};

#[derive(Parser)]
#[command(
    name = "defang",
    version,
    about = "Defang (or refang) URLs, domains, IPv4/IPv6 addresses, and email addresses",
    long_about = "\
Makes indicators of compromise (IoCs) inert for safe sharing by email or chat,
preventing mail clients and messaging apps from turning them into live hyperlinks.

Supported input types:
  URLs     https://evil.example.com  →  hxxps[://]evil[.]example[.]com
  Domains  evil.example.com          →  evil[.]example[.]com
  IPv4     192.168.1.1               →  192[.]168[.]1[.]1
  IPv6     2001:db8::1               →  2001[:]db8[:][:]1
  Email    user@evil.example.com     →  user[@]evil[.]example[.]com

Defanging is idempotent: running it on an already-defanged string is safe.

Examples:
  defang https://malware.example.com
  defang 192.168.1.1 user@phish.net 2001:db8::1
  defang --refang 'hxxps[://]malware[.]example[.]com'
  echo 'evil.example.com' | defang
  cat iocs.txt | defang --refang"
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
