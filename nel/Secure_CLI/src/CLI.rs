use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Secure_CLI")]
#[command(about = "A secure password manager CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Add {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    },
    List,
}