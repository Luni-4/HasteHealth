use crate::Config;
use haste_fhir_operation_error::{OperationOutcomeError, derive::OperationOutcomeError};

/// Configuration provider that loads settings from environment variables and `.env` files.
///
/// This implementation reads configuration from environment variables and supports loading
/// additional variables from `.env` files using the `dotenvy` crate. Files are loaded in
/// order, with later files potentially overriding earlier values.
pub struct EnvironmentConfig();

/// Errors that can occur when loading environment configuration.
#[derive(OperationOutcomeError, Debug)]
pub enum EnvironmentConfigError {
    /// Failed to parse or load a `.env` file.
    #[error(code = "invalid", diagnostic = "Invalid environment '{arg0}'!")]
    FailedToLoadEnvironment(#[from] dotenvy::Error),

    /// A required environment variable was not set or could not be read.
    #[error(
        code = "invalid",
        diagnostic = "Environment is misconfigured '{arg0}' for key '{arg1}'."
    )]
    EnvironmentVariableNotSet(std::env::VarError, String),
}

impl EnvironmentConfig {
    /// Creates a new environment configuration provider.
    ///
    /// Loads variables from the specified `.env` files. If multiple files are provided,
    /// they are loaded in order. Missing files are silently ignored, but parsing errors
    /// are propagated.
    ///
    /// # Arguments
    ///
    /// * `config_files` - Paths to `.env` files to load (e.g., `&[".env", ".env.development"]`).
    ///
    /// # Returns
    ///
    /// Returns a new `EnvironmentConfig` instance, or an error if file parsing fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = EnvironmentConfig::new(&[".env", ".env.development"])?;
    /// ```
    pub fn new(config_files: &[&str]) -> Result<Self, OperationOutcomeError> {
        for file in config_files {
            let _file_result = dotenvy::from_filename(file).map_err(EnvironmentConfigError::from);
        }

        Ok(EnvironmentConfig())
    }
}

impl<Key: Into<String>> Config<Key> for EnvironmentConfig {
    fn get(&self, key: Key) -> Result<String, OperationOutcomeError> {
        let key_string = key.into();
        let k = std::env::var(&key_string)
            .map_err(|e| EnvironmentConfigError::EnvironmentVariableNotSet(e, key_string))?;
        Ok(k)
    }
    fn set(&self, key: Key, value: String) -> Result<(), OperationOutcomeError> {
        unsafe {
            std::env::set_var(key.into(), value);
        }
        Ok(())
    }
}
