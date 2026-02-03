//! Dynamic value type for representing "any" JSON value.
//!
//! This module provides the [`Value`] type which can hold any valid JSON value.
//! It maps to the Arri "Empty Schema Form" (`{}`), which accepts any value.
//!
//! # Example
//!
//! ```rust
//! use arri_repr::{Value, NumberValue};
//! use std::collections::BTreeMap;
//!
//! // Create a Value with arbitrary structure
//! let mut data = BTreeMap::new();
//! data.insert("enabled".to_string(), Value::Bool(true));
//! data.insert("count".to_string(), Value::Number(NumberValue::PosInt(42)));
//!
//! let config_data = Value::Object(data);
//! ```

use crate::{Exportable, Serializable};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use std::collections::BTreeMap;
use std::fmt;

/// A dynamic value that can represent any valid JSON value.
///
/// This type is useful when you need to handle arbitrary JSON data without
/// knowing its structure at compile time. It exports to an empty Arri schema
/// (`{}`), which accepts any JSON value.
///
/// # Variants
///
/// - `Null` - Represents JSON `null`
/// - `Bool` - Represents JSON boolean (`true` or `false`)
/// - `Number` - Represents JSON numbers (integers and floats)
/// - `String` - Represents JSON strings
/// - `Array` - Represents JSON arrays
/// - `Object` - Represents JSON objects with string keys
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Value {
    #[default]
    /// JSON null value
    Null,
    /// JSON boolean value
    Bool(bool),
    /// JSON number value (integer or float)
    Number(NumberValue),
    /// JSON string value
    String(String),
    /// JSON array value
    Array(Vec<Self>),
    /// JSON object value with string keys
    Object(BTreeMap<String, Self>),
}

/// Represents a JSON number value.
///
/// JSON numbers can be positive integers, negative integers, or floating-point values.
/// This enum distinguishes between these cases to preserve precision.
#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
    /// A positive integer (including zero)
    PosInt(u64),
    /// A negative integer
    NegInt(i64),
    /// A floating-point number
    Float(f64),
}

/// Internal schema type for Value that serializes to `{}`
///
/// According to the Arri Type Definition spec, the "Empty Schema Form" is just `{}`,
/// which accepts any value and rejects nothing. This is equivalent to `any` in TypeScript.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct AnySchema {
    /// Optional metadata (description, deprecation, etc.)
    metadata: Option<crate::MetadataSchema>,
}

impl Serializable for AnySchema {
    fn serialize(&self) -> Option<String> {
        // If there's metadata, include it in the output
        if let Some(ref metadata) = self.metadata {
            crate::Serializer::builder()
                .set("metadata", metadata)
                .build()
                .into()
        } else {
            Some("{}".to_string())
        }
    }

    fn set_metadata(&mut self, metadata: crate::MetadataSchema) {
        self.metadata = Some(metadata);
    }
}

impl Exportable for Value {
    fn export_internal() -> impl Serializable {
        AnySchema::default()
    }
}

// =============================================================================
// Serde Serialize implementation
// =============================================================================

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(b) => serializer.serialize_bool(*b),
            Self::Number(n) => n.serialize(serializer),
            Self::String(s) => serializer.serialize_str(s),
            Self::Array(arr) => arr.serialize(serializer),
            Self::Object(obj) => obj.serialize(serializer),
        }
    }
}

impl Serialize for NumberValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PosInt(n) => serializer.serialize_u64(*n),
            Self::NegInt(n) => serializer.serialize_i64(*n),
            Self::Float(n) => serializer.serialize_f64(*n),
        }
    }
}

// =============================================================================
// Serde Deserialize implementation
// =============================================================================

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid JSON value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        if v >= 0 {
            Ok(Value::Number(NumberValue::PosInt(v as u64)))
        } else {
            Ok(Value::Number(NumberValue::NegInt(v)))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Value::Number(NumberValue::PosInt(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Value::Number(NumberValue::Float(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Value::String(v.to_string()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(Value::String(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some(elem) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(Value::Array(vec))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut btree = BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            btree.insert(key, value);
        }
        Ok(Value::Object(btree))
    }
}

impl<'de> Deserialize<'de> for NumberValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberVisitor)
    }
}

struct NumberVisitor;

impl<'de> Visitor<'de> for NumberVisitor {
    type Value = NumberValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a JSON number")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        if v >= 0 {
            Ok(NumberValue::PosInt(v as u64))
        } else {
            Ok(NumberValue::NegInt(v))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(NumberValue::PosInt(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(NumberValue::Float(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_schema_serializes_to_empty_object() {
        let schema = AnySchema::default();
        assert_eq!(schema.serialize(), Some("{}".to_string()));
    }

    #[test]
    fn test_value_export_returns_any_schema() {
        let export = Value::export();
        assert_eq!(export.serialize(), Some("{}".to_string()));
    }
}
