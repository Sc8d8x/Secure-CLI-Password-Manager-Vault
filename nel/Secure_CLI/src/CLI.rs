use clap::{Parser, Subcommand};
use comfy_table::Table;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "passman", about = "A secure CLI password manager")]
struct Cli {
    #[command(Subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Command {
    Add {
        #[arg(short, long)]
        service: String,
        #[arg(short, long)]
        username: String,
    }
    // list all password
    List,
    //  Initialize a new vault
    Init, 
 
}

fn get_vault_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("secure-passman");
    fs::create_dir_all(&path).unwrap();
    path.push("vault.json");
    path
}

fn prompt_password(prompt: &str) -> String {
    rpassword::prompt_password(prompt).unwrap()
}

