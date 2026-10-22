# Rust Secure Vault

A command-line password manager built with security-first principles in Rust. 
Designed to demonstrate the practical application of modern cryptographic standards, memory safety, and secure data handling.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Security Considerations

This project was built with a "security by design" approach. Key architectural decisions include:

1. **Memory-Hard Key Derivation**: Uses **Argon2** (the winner of the Password Hashing Competition) to derive the encryption key from the master password. This mitigates GPU/ASIC brute-force and dictionary attacks.
2. **Authenticated Encryption**: Utilizes **AES-256-GCM** (AEAD). This ensures both confidentiality and data integrity. Any tampering with the ciphertext will result in a decryption failure, preventing malicious modification of the vault.
3. **Memory Hygiene**: Implements the `zeroize` crate to explicitly wipe the master password and derived cryptographic keys from RAM immediately after use, mitigating cold-boot and memory-scraping attacks.
4. **Secure Randomness**: Relies on the OS-provided CSPRNG (`rand::thread_rng`) for generating salts and nonces, avoiding predictable pseudo-random number generators.
5. **No Custom Crypto**: Relies exclusively on well-audited, community-trusted cryptographic crates rather than implementing custom primitives.

## Features

- Initialize a new, securely encrypted local vault.
- Add, retrieve, and list credentials via a clean CLI interface.
- Hidden password input in the terminal (no echo).
- Automatic and explicit memory clearing of sensitive data.
- Structured JSON storage with base64-encoded ciphertext and nonce.

## Tech Stack

- **Language**: Rust
- **Cryptography**: `aes-gcm` (AES-256-GCM), `argon2` (Key Derivation), `rand` (CSPRNG)
- **Memory Safety**: `zeroize`
- **Serialization**: `serde`, `serde_json`
- **CLI Parsing**: `clap` (with derive macros)
- **Terminal I/O**: `rpassword` (for secure, hidden input)

## Usage
# 1. Initialize a new vault (prompts for a master password)
cargo run --release -- init

# 2. Add a new credential
cargo run --release -- add --site "github.com" --user "myuser" --pass "supersecret"

# 3. List all stored sites (passwords remain hidden)
cargo run --release -- list

# 4. Retrieve a specific credential (prompts for master password)
cargo run --release -- get --site "github.com"


### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/rust-secure-vault.git
cd rust-secure-vault

# Build in release mode for optimal performance
cargo build --release
