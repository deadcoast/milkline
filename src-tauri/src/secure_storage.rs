use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use keyring::Entry;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt;

const SERVICE_NAME: &str = "milk-player";
const ENCRYPTION_KEY_NAME: &str = "milk-encryption-key";

#[derive(Debug)]
pub enum StorageError {
    KeyringError(keyring::Error),
    EncryptionError(String),
    DecryptionError(String),
    Base64Error(base64::DecodeError),
}

impl From<keyring::Error> for StorageError {
    fn from(err: keyring::Error) -> Self {
        StorageError::KeyringError(err)
    }
}

impl From<base64::DecodeError> for StorageError {
    fn from(err: base64::DecodeError) -> Self {
        StorageError::Base64Error(err)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::KeyringError(e) => write!(f, "Keyring error: {}", e),
            StorageError::EncryptionError(e) => write!(f, "Encryption error: {}", e),
            StorageError::DecryptionError(e) => write!(f, "Decryption error: {}", e),
            StorageError::Base64Error(e) => write!(f, "Base64 error: {}", e),
        }
    }
}

impl std::error::Error for StorageError {}

/// Trait for secure credential storage
pub trait SecureStorage {
    /// Store a credential securely
    fn store(&self, key: &str, value: &str) -> Result<(), StorageError>;
    
    /// Retrieve a credential
    fn retrieve(&self, key: &str) -> Result<Option<String>, StorageError>;
    
    /// Delete a credential
    fn delete(&self, key: &str) -> Result<(), StorageError>;
}

/// Encrypted data structure
#[derive(Serialize, Deserialize)]
struct EncryptedData {
    nonce: String,
    ciphertext: String,
}

/// Platform-native secure storage implementation using Windows Credential Manager
pub struct PlatformSecureStorage;

impl PlatformSecureStorage {
    pub fn new() -> Self {
        PlatformSecureStorage
    }

    /// Get or create the encryption key for additional encryption layer
    fn get_or_create_encryption_key(&self) -> Result<Vec<u8>, StorageError> {
        let entry = Entry::new(SERVICE_NAME, ENCRYPTION_KEY_NAME).map_err(StorageError::KeyringError)?;
        
        match entry.get_password() {
            Ok(key_b64) => {
                // Decode existing key
                match general_purpose::STANDARD.decode(&key_b64) {
                    Ok(key) if key.len() == 32 => Ok(key),
                    _ => {
                        // Invalid key, regenerate
                        let mut key = vec![0u8; 32];
                        OsRng.fill_bytes(&mut key);
                        let key_b64 = general_purpose::STANDARD.encode(&key);
                        entry.set_password(&key_b64).map_err(StorageError::KeyringError)?;
                        Ok(key)
                    }
                }
            }
            Err(keyring::Error::NoEntry) | Err(keyring::Error::Ambiguous(_)) => {
                // Generate new key
                let mut key = vec![0u8; 32]; // 256-bit key
                OsRng.fill_bytes(&mut key);
                
                // Store it
                let key_b64 = general_purpose::STANDARD.encode(&key);
                entry.set_password(&key_b64).map_err(StorageError::KeyringError)?;
                
                Ok(key)
            }
            Err(e) => Err(StorageError::KeyringError(e)),
        }
    }

    /// Encrypt data using AES-256-GCM
    fn encrypt(&self, plaintext: &str) -> Result<String, StorageError> {
        let key_bytes = self.get_or_create_encryption_key()?;
        
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| StorageError::EncryptionError(e.to_string()))?;
        
        // Encode to base64
        let encrypted_data = EncryptedData {
            nonce: general_purpose::STANDARD.encode(nonce_bytes),
            ciphertext: general_purpose::STANDARD.encode(ciphertext),
        };
        
        serde_json::to_string(&encrypted_data)
            .map_err(|e| StorageError::EncryptionError(e.to_string()))
    }

    /// Decrypt data using AES-256-GCM
    fn decrypt(&self, encrypted: &str) -> Result<String, StorageError> {
        let key_bytes = self.get_or_create_encryption_key()?;
        
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        // Parse encrypted data
        let encrypted_data: EncryptedData = serde_json::from_str(encrypted)
            .map_err(|e| StorageError::DecryptionError(e.to_string()))?;
        
        // Decode from base64
        let nonce_bytes = general_purpose::STANDARD.decode(&encrypted_data.nonce)?;
        let ciphertext = general_purpose::STANDARD.decode(&encrypted_data.ciphertext)?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| StorageError::DecryptionError(e.to_string()))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| StorageError::DecryptionError(e.to_string()))
    }
}

impl SecureStorage for PlatformSecureStorage {
    fn store(&self, key: &str, value: &str) -> Result<(), StorageError> {
        // Encrypt the value
        let encrypted = self.encrypt(value)?;
        
        // Store in platform keyring
        let entry = Entry::new(SERVICE_NAME, key).map_err(StorageError::KeyringError)?;
        entry.set_password(&encrypted).map_err(StorageError::KeyringError)?;
        
        Ok(())
    }
    
    fn retrieve(&self, key: &str) -> Result<Option<String>, StorageError> {
        let entry = Entry::new(SERVICE_NAME, key).map_err(StorageError::KeyringError)?;
        
        match entry.get_password() {
            Ok(encrypted) => {
                // Decrypt the value
                let decrypted = self.decrypt(&encrypted)?;
                Ok(Some(decrypted))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(keyring::Error::Ambiguous(_)) => Ok(None), // Handle ambiguous entries
            Err(e) => Err(StorageError::KeyringError(e)),
        }
    }
    
    fn delete(&self, key: &str) -> Result<(), StorageError> {
        let entry = Entry::new(SERVICE_NAME, key).map_err(StorageError::KeyringError)?;
        entry.delete_credential().map_err(StorageError::KeyringError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let storage = PlatformSecureStorage::new();
        let test_key = "test_credential";
        let test_value = "secret_token_12345";
        
        // Store
        storage.store(test_key, test_value).unwrap();
        
        // Retrieve
        let retrieved = storage.retrieve(test_key).unwrap();
        assert_eq!(retrieved, Some(test_value.to_string()));
        
        // Cleanup
        storage.delete(test_key).unwrap();
    }

    #[test]
    fn test_retrieve_nonexistent() {
        let storage = PlatformSecureStorage::new();
        let result = storage.retrieve("nonexistent_key").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete() {
        let storage = PlatformSecureStorage::new();
        let test_key = "test_delete";
        let test_value = "delete_me";
        
        // Store
        storage.store(test_key, test_value).unwrap();
        
        // Delete
        storage.delete(test_key).unwrap();
        
        // Should not exist anymore
        let result = storage.retrieve(test_key).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_encryption_decryption() {
        let storage = PlatformSecureStorage::new();
        let plaintext = "sensitive_data_123";
        
        let encrypted = storage.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);
        
        let decrypted = storage.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_overwrite_credential() {
        let storage = PlatformSecureStorage::new();
        let test_key = "test_overwrite";
        
        // Store first value
        storage.store(test_key, "value1").unwrap();
        let retrieved1 = storage.retrieve(test_key).unwrap();
        assert_eq!(retrieved1, Some("value1".to_string()));
        
        // Overwrite with second value
        storage.store(test_key, "value2").unwrap();
        let retrieved2 = storage.retrieve(test_key).unwrap();
        assert_eq!(retrieved2, Some("value2".to_string()));
        
        // Cleanup
        storage.delete(test_key).unwrap();
    }
}
