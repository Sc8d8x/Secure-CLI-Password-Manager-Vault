use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
// Stroge one User 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct enterUser {
        pub salt: String,
        pub nonce: String,
        pub chiptext: Zeroize<String>,
}
// save on Disk (data encroptype)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SavedFile {
    #[serde(with = "base64_bytes")]
    pub salt: Vec<u8>,
     #[serde(with = "base64_bytes")]
    pub nonce: String,
     #[serde(with = "base64_bytes")]
    pub chiptext: String,
}


// Safe Stroge of the master password in Ram 
pub struct masterPossword(String);

impl masterPossword{
    
    pub fn new(pwd:String) -> Self {
        Self(Zeroizing::new(pwd));
    }
    // provides read-only access to the password bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
// rubbing
impl Drop for masterPossword {
    fn drop(&mut self){
        self.0.zeroize()
    }
}
