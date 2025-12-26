mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();
    println!("OSBN CLI started!");
}
