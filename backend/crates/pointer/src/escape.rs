//! Escaping and unescaping utilities for field names in paths.

/// Escapes special characters in a field name for safe path representation.
///
/// Replaces `~` with `~0` and `/` with `~1` to allow these characters in field names.
pub fn escape_field(field: &str) -> String {
    field.replace("~", "~0").replace("/", "~1")
}

/// Unescapes a field name that was previously escaped.
///
/// Replaces `~1` with `/` and `~0` with `~`.
pub fn unescape_field(field: &str) -> String {
    field.replace("~1", "/").replace("~0", "~")
}
