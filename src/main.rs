use clap::{Arg, Command};

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

    let input = matches.get_one::<String>("input");
    let output = matches.get_one::<String>("output");

    println!("Input: {:?}", input);
    println!("Output: {:?}", output);
    Ok(())
}
