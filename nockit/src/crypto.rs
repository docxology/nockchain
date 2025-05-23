//! Cryptographic utilities for nockit
//! 
//! Provides cryptographic functions, key management, and security utilities.

use anyhow::{Context, Result};
use blake3::Hasher;
use bs58;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Public key structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(pub Vec<u8>);

/// Private key structure (should be handled securely)
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateKey(pub Vec<u8>);

/// Key pair structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

/// Hash digest structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

/// Digital signature structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(pub Vec<u8>);

impl PublicKey {
    /// Create a new public key from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Get the raw bytes of the public key
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Encode the public key as base58
    pub fn to_base58(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
    
    /// Decode a public key from base58
    pub fn from_base58(encoded: &str) -> Result<Self> {
        let bytes = bs58::decode(encoded)
            .into_vec()
            .context("Failed to decode base58 public key")?;
        Ok(Self(bytes))
    }
    
    /// Verify a signature against this public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<bool> {
        // This is a placeholder implementation
        // In a real implementation, you'd use the appropriate cryptographic library
        // for the signature scheme being used (e.g., Ed25519, ECDSA, etc.)
        
        // For now, we'll just check if the signature is non-empty
        Ok(!signature.0.is_empty())
    }
}

impl PrivateKey {
    /// Create a new private key from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Get the raw bytes of the private key
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Sign a message with this private key
    pub fn sign(&self, message: &[u8]) -> Result<Signature> {
        // This is a placeholder implementation
        // In a real implementation, you'd use the appropriate cryptographic library
        
        // For now, we'll create a simple hash-based signature
        let mut hasher = Hasher::new();
        hasher.update(&self.0);
        hasher.update(message);
        let hash = hasher.finalize();
        
        Ok(Signature(hash.as_bytes().to_vec()))
    }
    
    /// Derive the public key from this private key
    pub fn public_key(&self) -> Result<PublicKey> {
        // This is a placeholder implementation
        // In a real implementation, you'd derive the public key using the appropriate algorithm
        
        // For now, we'll use a simple hash
        let mut hasher = Hasher::new();
        hasher.update(&self.0);
        hasher.update(b"public_key_derivation");
        let hash = hasher.finalize();
        
        Ok(PublicKey(hash.as_bytes().to_vec()))
    }
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Result<Self> {
        // This is a placeholder implementation
        // In a real implementation, you'd use a proper cryptographic random number generator
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher as StdHasher};
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let mut hasher = DefaultHasher::new();
        SystemTime::now().duration_since(UNIX_EPOCH)?.hash(&mut hasher);
        let seed = hasher.finish();
        
        let private_bytes = blake3::hash(&seed.to_le_bytes()).as_bytes().to_vec();
        let private_key = PrivateKey::from_bytes(private_bytes);
        let public_key = private_key.public_key()?;
        
        Ok(Self {
            public_key,
            private_key,
        })
    }
    
    /// Create a key pair from a private key
    pub fn from_private_key(private_key: PrivateKey) -> Result<Self> {
        let public_key = private_key.public_key()?;
        Ok(Self {
            public_key,
            private_key,
        })
    }
}

impl Hash {
    /// Create a new hash from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    /// Get the raw bytes of the hash
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    
    /// Encode the hash as hexadecimal
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    /// Decode a hash from hexadecimal
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        let bytes = hex::decode(hex_str)
            .context("Failed to decode hex hash")?;
        
        if bytes.len() != 32 {
            anyhow::bail!("Hash must be exactly 32 bytes, got {}", bytes.len());
        }
        
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&bytes);
        Ok(Self(hash_bytes))
    }
}

impl Signature {
    /// Create a new signature from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
    
    /// Get the raw bytes of the signature
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Encode the signature as base58
    pub fn to_base58(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
    
    /// Decode a signature from base58
    pub fn from_base58(encoded: &str) -> Result<Self> {
        let bytes = bs58::decode(encoded)
            .into_vec()
            .context("Failed to decode base58 signature")?;
        Ok(Self(bytes))
    }
}

/// Hash arbitrary data using BLAKE3
pub fn hash_data(data: &[u8]) -> Hash {
    let hash = blake3::hash(data);
    Hash(hash.into())
}

/// Hash multiple pieces of data together
pub fn hash_multiple(data_pieces: &[&[u8]]) -> Hash {
    let mut hasher = Hasher::new();
    for piece in data_pieces {
        hasher.update(piece);
    }
    Hash(hasher.finalize().into())
}

/// Verify a hash against data
pub fn verify_hash(data: &[u8], expected_hash: &Hash) -> bool {
    let computed_hash = hash_data(data);
    computed_hash == *expected_hash
}

/// Generate a random nonce
pub fn generate_nonce() -> Result<[u8; 32]> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher as StdHasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)?.hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    
    let seed = hasher.finish();
    let hash = blake3::hash(&seed.to_le_bytes());
    Ok(hash.into())
}

/// Derive a key from a password using PBKDF2-like function
pub fn derive_key_from_password(password: &str, salt: &[u8], iterations: u32) -> Result<[u8; 32]> {
    let mut hasher = Hasher::new();
    hasher.update(password.as_bytes());
    hasher.update(salt);
    
    let mut result = hasher.finalize();
    
    // Simple iteration (in practice, use a proper PBKDF2 implementation)
    for _ in 1..iterations {
        let mut hasher = Hasher::new();
        hasher.update(result.as_bytes());
        result = hasher.finalize();
    }
    
    Ok(result.into())
}

/// Validate a public key format
pub fn validate_public_key(pubkey: &str) -> Result<PublicKey> {
    PublicKey::from_base58(pubkey)
        .context("Invalid public key format")
}

/// Validate a signature format
pub fn validate_signature(signature: &str) -> Result<Signature> {
    Signature::from_base58(signature)
        .context("Invalid signature format")
}

// Display implementations

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PrivateKey([REDACTED])")
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_pair_generation() {
        let keypair = KeyPair::generate().unwrap();
        assert!(!keypair.public_key.as_bytes().is_empty());
        assert!(!keypair.private_key.as_bytes().is_empty());
    }

    #[test]
    fn test_public_key_base58() {
        let keypair = KeyPair::generate().unwrap();
        let encoded = keypair.public_key.to_base58();
        let decoded = PublicKey::from_base58(&encoded).unwrap();
        assert_eq!(keypair.public_key, decoded);
    }

    #[test]
    fn test_hash_data() {
        let data = b"hello world";
        let hash1 = hash_data(data);
        let hash2 = hash_data(data);
        assert_eq!(hash1, hash2);
        
        let different_data = b"hello world!";
        let hash3 = hash_data(different_data);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_signature() {
        let keypair = KeyPair::generate().unwrap();
        let message = b"test message";
        
        let signature = keypair.private_key.sign(message).unwrap();
        let is_valid = keypair.public_key.verify(message, &signature).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_hash_hex_encoding() {
        let data = b"test data";
        let hash = hash_data(data);
        let hex = hash.to_hex();
        let decoded = Hash::from_hex(&hex).unwrap();
        assert_eq!(hash, decoded);
    }
} 