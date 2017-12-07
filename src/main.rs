extern crate clap;
use clap::App;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Pulls data from Poloniex exchange.")
        .author("John B. <johnboydiv@gmail.com>")
        .args_from_usage("-n, --name=[NAME] 'Name of person to be greeted.'")
        .get_matches();

    let name = cli_args.value_of("name").unwrap_or("world");

    println!("Hello, {}!", name);
}
