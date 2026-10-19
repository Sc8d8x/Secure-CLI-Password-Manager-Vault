mod CLi;
mod Cryptography;
mod Data_Structures;

use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use rpassword::prompt_password;
use std::fs;
use zeroize::Zeroizing;

use CLi::{Cli, Command};
use Cryptography::Crypto;
use Data_Structures::{MasterPassword, PasswordEntry, SavedFile};

const VAULT_PATH: &str = "vault.json";

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            let pwd_str = prompt_password("Set master password: ")?; 
            let pwd = MasterPassword::new(pwd_str);
            
            let confirm_str = prompt_password("Confirm master password: ")?; 
            let confirm = MasterPassword::new(confirm_str);

            if pwd.as_bytes() != confirm.as_bytes() {
                eprintln!("Passwords do not match!");
                return Ok(());
            }

            let empty_entries: Vec<PasswordEntry> = Vec::new();
            let json_bytes = serde_json::to_vec(&empty_entries)?;
            
            let vault_file = Crypto::encrypt(&pwd, &json_bytes)?;
            let json = serde_json::to_string_pretty(&vault_file)?;
            
            fs::write(VAULT_PATH, json)?;
            println!("Vault initialized successfully.");
        }
        
        Command::List => {
            let pwd_str = prompt_password("Master password: ")?; 
            let pwd = MasterPassword::new(pwd_str);

            let vault_json = fs::read_to_string(VAULT_PATH)?;
            let vault_file: SavedFile = serde_json::from_str(&vault_json)?;
            
            let decrypted_bytes = Crypto::decrypt(&pwd, &vault_file)?;
            let entries: Vec<PasswordEntry> = serde_json::from_slice(&decrypted_bytes)?;

            let mut table = Table::new();
            table.set_header(vec!["Service", "Username"]);
            
            for entry in entries {
                table.add_row(vec![entry.service, entry.username]);
            }
            println!("{table}");
        }
        
        
        Command::Add { service, username } => { 
            let pwd_str = prompt_password("Master password: ")?;
            let pwd = MasterPassword::new(pwd_str);

            let password_str = prompt_password(&format!("Password for {}: ", service))?; 
            let password = Zeroizing::new(password_str);

            let vault_json = fs::read_to_string(VAULT_PATH)?;
            let vault_file: SavedFile = serde_json::from_str(&vault_json)?;
            
            let decrypted_bytes = Crypto::decrypt(&pwd, &vault_file)?;
            let mut entries: Vec<PasswordEntry> = serde_json::from_slice(&decrypted_bytes)?;

            entries.push(PasswordEntry {
                service,
                username,
                password,
            });

            let new_json_bytes = serde_json::to_vec(&entries)?;
            let new_vault_file = Crypto::encrypt(&pwd, &new_json_bytes)?;
            let new_json = serde_json::to_string_pretty(&new_vault_file)?;
            
            fs::write(VAULT_PATH, new_json)?;
            println!("Password added successfully.");
        }
    }

    Ok(())
}