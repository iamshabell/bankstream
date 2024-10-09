use std::process;

use clap::{Arg, Command};
use parser::Pain001Parser;

mod parser;
fn main() -> anyhow::Result<()> {
    let matches = Command::new("Bank Stream")
        .version("0.0.1")
        .about("Parses PAIN bank statement files")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file path"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output CSV file path"),
        )
        .get_matches();

    let buffer = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output");

    match Pain001Parser::new(buffer) {
        Ok(parser) => {
            if let Err(e) = parser.parse(output) {
                eprintln!("Error parsing file: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error initializing parser: {}", e);
            process::exit(1);
        }
    }

    Ok(())
}
