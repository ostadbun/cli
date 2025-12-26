use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "osbn")]
#[command(about = "OSBN CLI tool")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "login", about = "Log in via Google OAuth")]
    Login,
}
