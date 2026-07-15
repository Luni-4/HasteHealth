//! Pointer types for navigating and referencing MetaValue structures.
//!
//! This module provides `Path` for tracking field navigation paths and `TypedPointer`
//! for maintaining typed references to nested values with their root context.

use haste_reflect::MetaValue;
use std::{fmt::Display, sync::Arc};

mod escape;

/// A path string representing nested field access in a hierarchical structure.
///
/// Paths are built by descending through fields and array indices, and can be used
/// to retrieve values from a root MetaValue. Paths use `/` as a separator and escape
/// special characters (`~` and `/`) within field names.
#[derive(Debug, Clone)]
pub struct Path(String);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Path {
    /// Creates a new empty path.
    pub fn new() -> Self {
        Self("".to_string())
    }
    /// Descends into a field, appending it to the path.
    ///
    /// # Arguments
    ///
    /// * `field` - The field name to descend into. Special characters are automatically escaped.
    pub fn descend(&self, field: &str) -> Self {
        Self(format!("{}/{}", self.0, escape::escape_field(field)))
    }
    /// Ascends one level, returning the parent path and the current field key.
    ///
    /// Returns `None` if the path is already at the root (empty).
    pub fn ascend(&self) -> Option<(Self, Key)> {
        if self.0.is_empty() {
            None
        } else {
            let mut parts = self.0.rsplitn(2, '/');
            let field = parts.next().unwrap();
            let parent_path = parts.next().unwrap_or("");

            Some((
                Path(parent_path.to_string()),
                Key::from_str(&escape::unescape_field(field)),
            ))
        }
    }

    /// Retrieves a value from the root by following this path.
    ///
    /// # Arguments
    ///
    /// * `value` - The root MetaValue to traverse
    ///
    /// Returns `None` if any component of the path doesn't exist.
    pub fn get<'a>(&self, value: &'a dyn MetaValue) -> Option<&'a dyn MetaValue> {
        let mut current = value;
        // Skip the first empty part from the leading '/'
        for part in self.0.split('/').skip(1) {
            let k = Key::from_str(&escape::unescape_field(part));

            match k {
                Key::Field(field) => {
                    current = current.get_field(&field)?;
                }
                Key::Index(index) => {
                    current = current.get_index(index)?;
                }
            }
        }

        Some(current)
    }

    /// Retrieves and downcasts a value from the root by following this path.
    ///
    /// # Arguments
    ///
    /// * `value` - The root MetaValue to traverse
    ///
    /// # Generic Parameters
    ///
    /// * `Type` - The expected type of the value at the path
    ///
    /// Returns `None` if any component of the path doesn't exist or if the final value
    /// cannot be downcast to the specified type.
    pub fn get_typed<'a, Type: MetaValue>(&self, value: &'a dyn MetaValue) -> Option<&'a Type> {
        let current = self.get(value)?;
        current.as_any().downcast_ref::<Type>()
    }
}

/// A key for accessing a field or index in a hierarchical structure.
///
/// Keys can represent either a named field or a numeric array index.
#[derive(Debug)]
pub enum Key {
    /// A named field access.
    Field(String),
    /// An indexed array access.
    Index(usize),
}

impl Key {
    /// Parses a string into a Key, trying to interpret it as an index first.
    ///
    /// If the string parses as a `usize`, returns `Key::Index`; otherwise returns `Key::Field`.
    pub fn from_str(field: &str) -> Self {
        if let Ok(index) = field.parse::<usize>() {
            Key::Index(index)
        } else {
            Key::Field(field.to_string())
        }
    }
}

#[derive(Clone)]
struct ChildPointer<U>(*const U);

unsafe impl<U> Send for ChildPointer<U> {}
unsafe impl<U> Sync for ChildPointer<U> {}

/// A typed pointer to a value within a root MetaValue structure.
///
/// Maintains a reference to the root for lifetime management while pointing to a potentially nested value.
/// The pointer holds both a path (for debugging/identification) and a type-checked reference to the target value.
#[derive(Clone)]
pub struct TypedPointer<T: MetaValue, U: MetaValue> {
    root: Arc<T>,
    value: ChildPointer<U>,
    path: Path,
}

impl<Root: MetaValue, U: MetaValue> TypedPointer<Root, U> {
    /// Creates a new TypedPointer to the root of a value.
    ///
    /// # Arguments
    ///
    /// * `value` - The root MetaValue wrapped in an Arc for shared ownership
    pub fn new(value: Arc<Root>) -> TypedPointer<Root, Root> {
        TypedPointer {
            value: ChildPointer(&*value.as_ref() as *const Root),
            root: value,
            path: Path::new(),
        }
    }

    /// Creates a new pointer to the root, resetting the path.
    pub fn root(&self) -> TypedPointer<Root, Root> {
        TypedPointer {
            value: ChildPointer(&*self.root.as_ref() as *const Root),
            root: self.root.clone(),
            path: Path::new(),
        }
    }

    /// Returns the path as a string.
    pub fn path(&self) -> &str {
        self.path.0.as_str()
    }

    /// Retrieves the current value if the pointer is still valid.
    pub fn value(&self) -> Option<&U> {
        let p = unsafe { (*self.value.0).as_any().downcast_ref::<U>() };
        p
    }

    /// Descends into a child field or array index, returning a typed pointer to the child.
    ///
    /// # Arguments
    ///
    /// * `field` - The key (field or index) to descend into
    ///
    /// # Generic Parameters
    ///
    /// * `Child` - The expected type of the child value
    ///
    /// Returns `None` if the field doesn't exist or cannot be downcast to the Child type.
    pub fn descend<Child: MetaValue>(&self, field: &Key) -> Option<TypedPointer<Root, Child>> {
        match field {
            Key::Field(field) => self.value().and_then(|v| {
                v.get_field(field)
                    .and_then(|v| v.as_any().downcast_ref::<Child>())
                    .map(|child| TypedPointer {
                        root: self.root.clone(),
                        value: ChildPointer(&*child as *const Child),
                        path: self.path.descend(field),
                    })
            }),
            Key::Index(index) => self.value().and_then(|v| {
                v.get_index(*index)
                    .and_then(|v| v.as_any().downcast_ref::<Child>())
                    .map(|child| TypedPointer {
                        root: self.root.clone(),
                        value: ChildPointer(&*child as *const Child),
                        path: self.path.descend(&index.to_string()),
                    })
            }),
        }
    }

    /// Ascends one level, returning the parent path and current key.
    pub fn ascend(&self) -> Option<(Path, Key)> {
        self.path.ascend()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use haste_fhir_model::r4::generated::{
        resources::Patient, types::FHIRString, types::HumanName,
    };

    #[test]
    fn test_pointer_descend() {
        let patient = Arc::new(Patient {
            id: Some("patient-1".to_string()),
            name: Some(vec![Box::new(HumanName {
                family: Some(Box::new(FHIRString {
                    value: Some("Doe".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            })]),
            ..Default::default()
        });

        let pointer = TypedPointer::<Patient, Patient>::new(patient);
        let pointer = pointer
            .descend::<Vec<Box<HumanName>>>(&Key::Field("name".to_string()))
            .unwrap();
        assert_eq!(pointer.path(), "/name");
        let pointer = pointer.descend::<Box<HumanName>>(&Key::Index(0)).unwrap();
        assert_eq!(pointer.path(), "/name/0");
        let pointer = pointer
            .descend::<Box<FHIRString>>(&Key::Field("family".to_string()))
            .unwrap();
        let pointer = pointer
            .descend::<String>(&Key::Field("value".to_string()))
            .unwrap();

        assert_eq!(pointer.path(), "/name/0/family/value");
        assert_eq!(pointer.value(), Some(&"Doe".to_string()));
    }

    #[test]
    fn test_path() {
        let patient = Arc::new(Patient {
            id: Some("patient-1".to_string()),
            name: Some(vec![Box::new(HumanName {
                family: Some(Box::new(FHIRString {
                    value: Some("Doe".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            })]),
            ..Default::default()
        });

        let path = Path::new()
            .descend("name")
            .descend("0")
            .descend("family")
            .descend("value");

        assert_eq!(path.0, "/name/0/family/value");
        let k = path.get_typed::<String>(patient.as_ref());

        assert_eq!(k, Some(&"Doe".to_string()));
    }
}
