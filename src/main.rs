extern crate clap;
use clap::{App, SubCommand};

mod polydiv;
mod ascii85;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Does some basic math things.")
        .author("John B. <johnboydiv@gmail.com>")
        .subcommand(SubCommand::with_name("ascii85").about(
            "Decode/Encode a text string from/to ascii85",
        ))
        .subcommand(SubCommand::with_name("polydiv").about(
            "Perform polynomial division",
        ))
        .get_matches();

    // Determine which subcommand was chosen
    match cli_args.subcommand_name() {
        Some("polydiv") => polydiv::polynomial_division(),
        Some("ascii85") => ascii85::ascii85_codec(),
        _ => {
            println!("Error: No subcommand provided.");
            println!("Run 'rusty-math -h' for a list of available commands.");
        }
    }
}
