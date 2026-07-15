use std::{any::Any, fmt::Debug};

/// Core trait for runtime reflection on structured values.
///
/// Implementors can introspect their fields and indices at runtime, retrieve nested values,
/// and downcast to concrete types. This enables generic traversal algorithms and dynamic type checking.
///
/// # Contract
///
/// - `fields()` should return the names of all accessible fields
/// - `get_field()` and `get_index()` must handle out-of-bounds gracefully by returning `None`
/// - `fhir_type()` should return a stable, unique identifier for the type
/// - `as_any()` enables downcasting to the concrete type via `downcast_ref()`
/// - `flatten()` should return all leaf values in depth-first order
pub trait MetaValue: Any + Debug + Send + Sync {
    /// Returns the names of all fields in this value.
    fn fields(&self) -> Vec<&'static str>;

    /// Returns an immutable reference to a field by name, if it exists.
    fn get_field<'a>(&'a self, field: &str) -> Option<&'a dyn MetaValue>;
    /// Returns a mutable reference to a field by name, if it exists.
    fn get_field_mut<'a>(&'a mut self, field: &str) -> Option<&'a mut dyn MetaValue>;

    /// Returns an immutable reference to an indexed element, if the index is valid.
    fn get_index<'a>(&'a self, index: usize) -> Option<&'a dyn MetaValue>;
    /// Returns a mutable reference to an indexed element, if the index is valid.
    fn get_index_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut dyn MetaValue>;

    /// Returns all leaf values in this structure in depth-first order.
    fn flatten(&self) -> Vec<&dyn MetaValue>;

    /// Returns this value as `Any` for downcasting to concrete types.
    fn as_any(&self) -> &dyn Any;

    /// Returns a stable FHIR type identifier for this value (e.g., "http://hl7.org/fhirpath/System.String").
    fn fhir_type(&self) -> &'static str;

    /// Returns true if this value represents multiple items (e.g., a collection), false for scalars.
    fn is_many(&self) -> bool;
}
