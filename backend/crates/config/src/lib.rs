use crate::environment::EnvironmentConfig;
use haste_fhir_operation_error::OperationOutcomeError;
use std::sync::Arc;

mod environment;

/// Abstraction for retrieving and setting configuration values.
///
/// This trait provides a unified interface for different configuration backends
/// (environment variables, files, etc.). Implementations must be thread-safe.
///
/// # Type Parameters
///
/// * `Key` - The key type to identify configuration values, typically a string-like type.
pub trait Config<Key: Into<String>>: Send + Sync {
    /// Retrieves a configuration value by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The configuration key to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the configuration value as a string, or an error if not found.
    fn get(&self, name: Key) -> Result<String, OperationOutcomeError>;

    /// Sets a configuration value.
    ///
    /// # Arguments
    ///
    /// * `name` - The configuration key to set.
    /// * `value` - The value to set for this key.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the operation fails.
    fn set(&self, name: Key, value: String) -> Result<(), OperationOutcomeError>;
}

/// Enumeration of supported configuration backends.
#[derive(Clone, Copy, Debug)]
pub enum ConfigType {
    /// Load configuration from environment variables and `.env` files.
    Environment,
}

impl From<&str> for ConfigType {
    fn from(value: &str) -> Self {
        match value {
            "environment" => ConfigType::Environment,
            _ => panic!("Unknown config type"),
        }
    }
}

/// Creates a configuration provider of the specified type.
///
/// # Arguments
///
/// * `config_type` - The type of configuration backend to create.
///
/// # Returns
///
/// Returns an arc-wrapped trait object implementing the `Config` trait.
/// The provider is ready to use immediately.
pub fn get_config<Key: Into<String>>(config_type: ConfigType) -> Arc<dyn Config<Key>> {
    match config_type {
        ConfigType::Environment => {
            Arc::new(EnvironmentConfig::new(&[".env", ".env.development"]).unwrap())
        }
    }
}
