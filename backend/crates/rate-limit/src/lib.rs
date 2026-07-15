//! Rate limiting trait and error types for controlling request throughput.
//!
//! Provides a generic rate limiting interface that can be implemented by various backends
//! (e.g., Redis, in-memory) to enforce limits on resources identified by string keys.

use std::future::Future;
use std::pin::Pin;

/// Error type for rate limit operations.
#[derive(Debug)]
pub enum RateLimitError {
    /// A general error occurred, with a descriptive message.
    Error(String),
    /// The rate limit has been exceeded.
    Exceeded,
}

/// Trait for rate limiting implementations.
///
/// Implementors should track requests by key and enforce per-window limits.
/// Returns the number of remaining points/requests after the check.
pub trait RateLimit: Sync + Send {
    /// Checks and applies a rate limit.
    ///
    /// # Arguments
    ///
    /// * `rate_key` - The identifier for this rate limit (e.g., user ID, IP address)
    /// * `max` - The maximum number of requests allowed in the window
    /// * `points` - The number of points/requests to consume (typically 1)
    /// * `window_in_seconds` - The time window in seconds for this limit
    ///
    /// # Returns
    ///
    /// * `Ok(remaining)` - The number of remaining requests if the limit was not exceeded
    /// * `Err(RateLimitError)` - An error if the operation failed or the limit was exceeded
    fn check<'a>(
        &'a self,
        rate_key: &'a str,
        max: i32,
        points: i32,
        window_in_seconds: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, RateLimitError>> + Send + 'a>>;
}
