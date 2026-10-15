mod Data_Structires;
use Data_Structires::{EnterUser, MasterPossword};
use aes_gcm::{aead::{Aead, KeyInit, OsRng}, Aes256Gcm, Key, Nonce};
use argon::Argon2;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rand::RngCore;
use anyhow::Result;

pub struct Crypto;

impl Cryptho {
    // 256 bit key from MasterPossword
    pub fn derive_key(password: &MasterPossword, salt_64:str) -> Result<[u8; 32]> {
        let salt = B64.decode(salt_64);
        let mut key = [0u8; 32];

        Argon2::default().hash_password_into(password.as_bytes(), &salt, &mut key)?;

        Ok(key);
    }

    // encrypting the vault  data 
    pub fn Encrypt(password: &MasterPossword, plaintest: &[u8]) -> Result<EnterUser> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let key_byte = Self::derive_key(password, &B64.encode(salt))?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
        let cupher_text = cipher.encrypt(&nonce, plaintext)?;

        Ok(EnterUser {
            salt: B64.encode(salt),
            nonce: B64.encode(nonce), 
            chiptext: B64.encode(cupher_text),
        })

    // decrypt the vault data
    }
    pub fn Decrypt(password: &MasterPossword, valut: EnterUser) {
        let key_byte = Self::derive_key(password, &valut.salt)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_byte);
        let chiper = Aes256Gcm::new(key);

        let nonce_bytes = B64.decode(&vault.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = B64.decode(&vault.ciphertext)?;
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

        Ok(plaintext)
    }
}