//! Runtime reflection and type introspection for FHIR and other MetaValue types.
//!
//! This crate provides traits and implementations for runtime access to structured data,
//! enabling field inspection, traversal, and type-safe downcasting without compile-time type information.

mod primitives;
mod traits;

pub use traits::*;

/// Procedural macros for deriving MetaValue trait implementations.
///
/// Only available when the `derive` feature is enabled.
#[cfg(feature = "derive")]
pub mod derive;
