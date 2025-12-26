mod cli;
mod commands;

use clap::Parser;
use cli::Cli;

fn main() {
    let _cli = Cli::parse();

    commands::login::run();
}
