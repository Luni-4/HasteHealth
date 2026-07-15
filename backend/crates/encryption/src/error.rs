use haste_fhir_operation_error::derive::OperationOutcomeError;

/// Errors that can occur during encryption, decryption, or secret retrieval operations.
#[derive(OperationOutcomeError, Debug)]
pub enum EncryptionError {
    /// The requested secret was not found in the provider's backend.
    #[error(code = "not-found", diagnostic = "Secret '{arg0}' was not found")]
    SecretNotFound(String),

    /// Failed to retrieve a secret from the backend (network error, permission denied, etc.).
    #[error(
        code = "exception",
        diagnostic = "Failed to retrieve secret '{arg0}': {arg1}"
    )]
    SecretRetrievalFailed(String, String),

    /// The encryption key has an invalid length for the cipher suite.
    #[error(
        code = "invalid",
        diagnostic = "Encryption key must be {arg0} bytes, got {arg1}"
    )]
    InvalidKeyLength(usize, usize),

    /// The encryption operation failed.
    #[error(code = "exception", diagnostic = "Encryption failed: {arg0}")]
    EncryptionFailed(String),

    /// The decryption operation failed (invalid ciphertext, tag mismatch, etc.).
    #[error(code = "exception", diagnostic = "Decryption failed: {arg0}")]
    DecryptionFailed(String),

    /// The requested secrets provider type is not yet implemented.
    #[error(
        code = "not-supported",
        diagnostic = "Secrets provider '{arg0}' is not yet implemented"
    )]
    ProviderNotImplemented(String),
}
