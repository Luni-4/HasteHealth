//! FHIR R4 (Release 4) resources and types.
//!
//! This module provides the complete FHIR R4 specification including resources,
//! data types, terminology, and conversion utilities.

pub mod conversion;
pub mod datetime;
pub mod generated;
#[cfg(feature = "sqlx")]
pub mod sqlx;
