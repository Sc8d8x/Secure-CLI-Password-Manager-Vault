use anyhow::Result;
use argon2::Argon2;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use rand::RngCore; 

use crate::Data_Structures::{MasterPassword, SavedFile};

pub struct Crypto;

impl Crypto {
    
// 256 bit key from MasterPossword
    pub fn derive_key(password: &MasterPassword, salt: &[u8]) -> Result<[u8; 32]> {
        let mut key_bytes = [0u8; 32];
        
        
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key_bytes)
            .map_err(|e| anyhow::anyhow!("Argon2 key derivation failed: {}", e))?;
        
        Ok(key_bytes)
    }
    // encrypting the vault  data 
    pub fn encrypt(password: &MasterPassword, plaintext: &[u8]) -> Result<SavedFile> {
        
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        let key_bytes = Self::derive_key(password, &salt)?;

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 

        let ciphertext = cipher.encrypt(&nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        Ok(SavedFile {
            salt: salt.to_vec(),
            nonce: nonce.to_vec(),
            ciphertext,
        })
    }
    // decrypt the vault data
    pub fn decrypt(password: &MasterPassword, vault: &SavedFile) -> Result<Vec<u8>> {
        
        let key_bytes = Self::derive_key(password, &vault.salt)?;

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        let nonce = aes_gcm::Nonce::from_slice(&vault.nonce);
        let plaintext = cipher.decrypt(nonce, vault.ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        Ok(plaintext)
    }
}

