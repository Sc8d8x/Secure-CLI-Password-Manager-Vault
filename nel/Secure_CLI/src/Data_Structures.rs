use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;
// save one user
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    #[serde(with = "zeroizing_string")]
    pub password: Zeroizing<String>,
}

// save on Disk (data encroptype)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnterUser {
    pub salt: String,
    pub nonce: String,
    #[serde(with = "zeroizing_string")]
    pub ciphertext: Zeroizing<String>,
}
// disk representation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedFile {
    #[serde(with = "base64_bytes")]
    pub salt: Vec<u8>,
    #[serde(with = "base64_bytes")]
    pub nonce: Vec<u8>,
    #[serde(with = "base64_bytes")]
    pub ciphertext: Vec<u8>,
}

// Safe Storage of the master password in RAM
pub struct MasterPassword(Zeroizing<String>);

impl MasterPassword {
    pub fn new(pwd: String) -> Self {
        Self(Zeroizing::new(pwd)) 
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

mod zeroizing_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use zeroize::Zeroizing;

    pub fn serialize<S>(val: &Zeroizing<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        val.as_str().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Zeroizing<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        
        let s = String::deserialize(deserializer)?;
        Ok(Zeroizing::new(s))
    }
}

// serde helper for Vec<u8>  base64 string
mod base64_bytes {
    use serde::{Deserialize, Deserializer, Serializer}; 
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&BASE64.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BASE64.decode(s).map_err(serde::de::Error::custom)
    }
}