mod error;

/// Symmetric encryption/decryption implementations for various cipher suites.
pub mod encryption;

/// Secrets provider implementations (environment, AWS, GCP, etc.).
pub mod providers;

/// Core traits for encryption and secrets management.
pub mod traits;

pub use error::EncryptionError;
pub use traits::{Encryptor, Secret, SecretsProvider};

use std::sync::Arc;

/// Enumeration of supported secrets provider backends.
///
/// Each variant represents a different source for retrieving encryption keys
/// and other secret material.
#[derive(Clone, Debug)]
pub enum SecretsProviderKind {
    /// Load secrets from environment variables, optionally with a prefix.
    ///
    /// Variable names are constructed as `{prefix}_{secret_name}` if a prefix is provided,
    /// otherwise just `{secret_name}`.
    Environment {
        /// Optional prefix for environment variable names.
        prefix: Option<String>,
    },
    /// Load secrets from AWS Secrets Manager.
    #[cfg(feature = "aws")]
    Aws {
        /// AWS region where secrets are stored.
        region: String,
    },
    /// Load secrets from Google Cloud Secret Manager.
    #[cfg(feature = "gcp")]
    Gcp {
        /// GCP project ID containing the secrets.
        project_id: String,
    },
}

/// Creates a secrets provider of the specified kind.
///
/// Returns an arc-wrapped trait object implementing the `SecretsProvider` trait.
/// The provider is ready to use immediately.
///
/// # Arguments
///
/// * `kind` - The type of secrets provider backend to create.
///
/// # Returns
///
/// Returns an arc-wrapped provider that can be used to retrieve secrets.
pub fn get_secrets_provider(kind: SecretsProviderKind) -> Arc<dyn SecretsProvider> {
    match kind {
        SecretsProviderKind::Environment { prefix } => Arc::new(
            providers::environment::EnvironmentSecretsProvider::new(prefix),
        ),
        #[cfg(feature = "aws")]
        SecretsProviderKind::Aws { region } => {
            Arc::new(providers::aws::AwsSecretsManagerProvider::new(region))
        }
        #[cfg(feature = "gcp")]
        SecretsProviderKind::Gcp { project_id } => {
            Arc::new(providers::gcp::GcpSecretManagerProvider::new(project_id))
        }
    }
}
