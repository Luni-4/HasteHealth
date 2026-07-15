use haste_fhir_operation_error::OperationOutcomeError;
use std::{future::Future, pin::Pin};

/// A secret's raw byte value. `Debug` is redacted so the value never
/// ends up in logs or error messages by accident.
///
/// This type provides a safe wrapper around sensitive byte data (encryption keys,
/// credentials, etc.) that ensures the actual value is never accidentally logged.
pub struct Secret(Vec<u8>);

impl Secret {
    /// Creates a new secret from a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The raw secret material to store.
    ///
    /// # Returns
    ///
    /// A new `Secret` instance wrapping the provided bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Exposes the raw bytes of the secret.
    ///
    /// Use this method only when you need to actually use the secret value
    /// (e.g., to initialize an encryption cipher). Avoid passing this reference
    /// to untrusted code or functions that might log it.
    ///
    /// # Returns
    ///
    /// A byte slice containing the secret material.
    pub fn expose_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret(REDACTED)")
    }
}

/// Retrieves secret material (encryption keys, credentials, etc.) by name
/// from a backing store, e.g. AWS Secrets Manager, GCP Secret Manager, or
/// environment variables.
///
/// Implementations must be async and thread-safe. Secrets should be cached
/// where possible to avoid repeated backend calls.
pub trait SecretsProvider: Sync + Send {
    /// Retrieves a secret by name from the backing store.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the secret to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the secret wrapped in `Ok`, or an `OperationOutcomeError` if the
    /// secret cannot be found or retrieved.
    fn get_secret<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<Secret, OperationOutcomeError>> + Send + 'a>>;
}

/// The result of an encryption operation.
///
/// Contains both the nonce (initialization vector) and the ciphertext.
/// Both must be stored/transmitted together for decryption to succeed.
pub struct EncryptionResult {
    /// The nonce (IV) used for encryption. Must be unique for each plaintext encrypted with the same key.
    pub nonce: Vec<u8>,
    /// The encrypted ciphertext.
    pub ciphertext: Vec<u8>,
}

/// Symmetric encryption/decryption of arbitrary byte payloads.
///
/// Implementations manage their own encryption keys (e.g., fetching from a `SecretsProvider`)
/// and handle the cipher suite details. All implementations must be thread-safe.
pub trait Encryptor: Sync + Send {
    /// Encrypts the provided plaintext.
    ///
    /// # Arguments
    ///
    /// * `plaintext` - The raw bytes to encrypt.
    ///
    /// # Returns
    ///
    /// Returns an `EncryptionResult` containing the nonce and ciphertext,
    /// or an error if encryption fails.
    fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptionResult, OperationOutcomeError>;

    /// Decrypts the provided ciphertext.
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - An `EncryptionResult` containing the nonce and ciphertext to decrypt.
    ///
    /// # Returns
    ///
    /// Returns the original plaintext as a vector of bytes,
    /// or an error if decryption fails (invalid tag, corrupted data, etc.).
    fn decrypt(&self, ciphertext: &EncryptionResult) -> Result<Vec<u8>, OperationOutcomeError>;
}
