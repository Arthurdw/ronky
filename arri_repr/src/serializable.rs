// TODO: create a macro which automatically generates this implementation with a derive
use std::{any::type_name, collections::HashMap, fmt::Debug};

use downcast_rs::{Downcast, impl_downcast};

use crate::{MetadataSchema, serializer::Serializer};

/// Escapes special characters in a string for JSON output.
///
/// Follows the JSON specification for string escaping:
/// - `"` -> `\"`
/// - `\` -> `\\`
/// - `/` -> `\/`
/// - Backspace -> `\b`
/// - Form feed -> `\f`
/// - Newline -> `\n`
/// - Carriage return -> `\r`
/// - Tab -> `\t`
/// - Control characters and other special Unicode -> `\uXXXX`
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 16);
    result.push('"');

    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '/' => result.push_str("\\/"),
            '\u{0008}' => result.push_str("\\b"), // backspace
            '\u{000C}' => result.push_str("\\f"), // form feed
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            ch if ch.is_control() => {
                // Escape other control characters as \uXXXX
                result.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => result.push(ch),
        }
    }

    result.push('"');
    result
}

/// Triggers a panic with a detailed error message, including the type, serialized data,
/// and value that caused the issue. This function is intended to report bugs.
///
/// # Parameters
/// - `message`: A message describing the context of the panic.
/// - `serialized`: The serialized representation of the data.
/// - `value`: The original value that caused the issue.
///
/// # Panics
/// This function always panics with a formatted message containing the provided details.
///
/// # Note
/// The panic message includes a link to report bugs, encouraging users to provide feedback.
fn do_panic<T>(message: impl std::fmt::Display, serialized: impl Debug, value: impl Debug) -> !
where
    T: ?Sized,
{
    panic!(
        "{}!\n\
        This is a bug, please report it @ <https://github.com/Arthurdw/ronky/issues>\n\
        Type: {:?}\n\
        Serialized: {:?}\n\
        Value: {:?}",
        message,
        type_name::<T>(),
        serialized,
        value
    );
}

/// A trait for types that can be serialized into a string representation.
///
/// This trait also provides default implementations for setting metadata,
/// nullability, and renaming, which trigger a panic if not implemented.
pub trait Serializable: Downcast {
    /// Serializes the object into an optional string representation.
    ///
    /// # Returns
    /// An `Option<String>` containing the serialized representation, or `None` if serialization fails.
    fn serialize(&self) -> Option<String>;

    /// Sets metadata for the object.
    ///
    /// # Arguments
    /// - `metadata`: The metadata to set.
    ///
    /// # Panics
    /// This method panics if not implemented for the type.
    fn set_metadata(&mut self, metadata: MetadataSchema) {
        do_panic::<Self>(
            "set_metadata is not implemented for this type",
            self.serialize(),
            metadata,
        );
    }

    /// Sets the nullability of the object.
    ///
    /// # Arguments
    /// - `nullable`: A boolean indicating whether the object is nullable.
    ///
    /// # Panics
    /// This method panics if not implemented for the type.
    fn set_nullable(&mut self, nullable: bool) {
        do_panic::<Self>(
            "set_nullable is not implemented for this type",
            self.serialize(),
            nullable,
        );
    }

    /// Renames the object.
    ///
    /// # Arguments
    /// - `new_name`: The new name to assign to the object.
    ///
    /// # Panics
    /// This method panics if not implemented for the type.
    fn set_rename(&mut self, new_name: &str) {
        do_panic::<Self>(
            "set_rename is not implemented for this type",
            self.serialize(),
            new_name,
        );
    }
}

impl_downcast!(Serializable);

impl Debug for dyn Serializable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize().unwrap_or("undefined".to_string()))
    }
}

impl PartialEq for dyn Serializable {
    fn eq(&self, other: &Self) -> bool {
        self.serialize() == other.serialize()
    }
}

impl Eq for dyn Serializable {}

impl Serializable for &'static str {
    fn serialize(&self) -> Option<String> {
        escape_json_string(self).into()
    }
}

impl Serializable for String {
    fn serialize(&self) -> Option<String> {
        escape_json_string(self).into()
    }
}

impl Serializable for bool {
    fn serialize(&self) -> Option<String> {
        self.to_string().into()
    }
}

// Macro to generate Serializable implementations for numeric types
macro_rules! impl_serializable_for_numeric {
    ($($type:ty),* $(,)?) => {
        $(
            impl Serializable for $type {
                fn serialize(&self) -> Option<String> {
                    self.to_string().into()
                }
            }
        )*
    };
}

// Numeric type implementations
impl_serializable_for_numeric!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);

impl Serializable for () {
    fn serialize(&self) -> Option<String> {
        "null".to_string().into()
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Option<String> {
        let serialized_elements: Vec<String> = self.iter().filter_map(|e| e.serialize()).collect();
        format!("[{}]", serialized_elements.join(",")).into()
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self) -> Option<String> {
        self.as_ref().and_then(|value| value.serialize())
    }
}

impl<T: Serializable> Serializable for HashMap<String, T> {
    fn serialize(&self) -> Option<String> {
        self.iter()
            .fold(Serializer::builder(), |mut builder, (key, value)| {
                builder.set(key, value);
                builder
            })
            .build()
            .into()
    }
}

impl<T: Serializable> Serializable for indexmap::IndexMap<String, T> {
    fn serialize(&self) -> Option<String> {
        self.iter()
            .fold(Serializer::builder(), |mut builder, (key, value)| {
                builder.set(key, value);
                builder
            })
            .build()
            .into()
    }
}

impl<T: Serializable> Serializable for Box<T> {
    fn serialize(&self) -> Option<String> {
        self.as_ref().serialize()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.as_mut().set_metadata(metadata)
    }

    fn set_nullable(&mut self, nullable: bool) {
        self.as_mut().set_nullable(nullable)
    }

    fn set_rename(&mut self, new_name: &str) {
        self.as_mut().set_rename(new_name)
    }
}

impl Serializable for Box<dyn Serializable> {
    fn serialize(&self) -> Option<String> {
        self.as_ref().serialize()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.as_mut().set_metadata(metadata)
    }

    fn set_nullable(&mut self, nullable: bool) {
        self.as_mut().set_nullable(nullable)
    }

    fn set_rename(&mut self, new_name: &str) {
        self.as_mut().set_rename(new_name)
    }
}

#[cfg(feature = "chrono")]
impl Serializable for chrono::DateTime<chrono::FixedOffset> {
    fn serialize(&self) -> Option<String> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
            .serialize()
    }
}

#[cfg(feature = "chrono")]
impl Serializable for chrono::DateTime<chrono::Utc> {
    fn serialize(&self) -> Option<String> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
            .serialize()
    }
}

// TODO: implement other features serialization and general object serialization

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MockSerializable {
        value: String,
    }

    impl Serializable for MockSerializable {
        fn serialize(&self) -> Option<String> {
            self.value.serialize()
        }
    }

    #[test]
    fn test_serialize() {
        let mock = MockSerializable {
            value: "test_value".to_string(),
        };
        assert_eq!(mock.serialize(), Some("\"test_value\"".to_string()));
    }

    #[test]
    fn test_debug_trait() {
        let mock = MockSerializable {
            value: "debug_value".to_string(),
        };
        assert_eq!(
            format!("{:?}", &mock as &dyn Serializable),
            "\"debug_value\""
        );
    }

    #[test]
    fn test_partial_eq_trait() {
        let mock1 = MockSerializable {
            value: "value".to_string(),
        };
        let mock2 = MockSerializable {
            value: "value".to_string(),
        };
        assert_eq!(&mock1 as &dyn Serializable, &mock2 as &dyn Serializable);
    }

    #[test]
    fn test_serialize_str() {
        let value = "Hello, world!";
        assert_eq!(value.serialize(), Some("\"Hello, world!\"".to_string()));
    }

    #[test]
    fn test_serialize_string() {
        let value = "Hello, world!".to_string();
        assert_eq!(value.serialize(), Some("\"Hello, world!\"".to_string()));
    }

    #[test]
    fn test_serialize_vec() {
        let vec = vec![
            MockSerializable {
                value: "value1".to_string(),
            },
            MockSerializable {
                value: "value2".to_string(),
            },
        ];
        let serialized: serde_json::Value =
            serde_json::from_str(&vec.serialize().unwrap()).unwrap();

        assert_eq!(serialized, serde_json::json!(["value1", "value2"]));
    }

    #[test]
    fn test_serialize_option() {
        let value = Some(MockSerializable {
            value: "optional_value".to_string(),
        });

        let none_value: Option<MockSerializable> = None;

        assert_eq!(value.serialize(), Some("\"optional_value\"".to_string()));
        assert_eq!(none_value.serialize(), None);
    }

    #[test]
    fn test_recursive_serialize_vec() {
        let vec = vec![
            MockSerializable {
                value: "value1".to_string(),
            },
            MockSerializable {
                value: "value2".to_string(),
            },
        ];

        let serialized: serde_json::Value =
            serde_json::from_str(&vec.serialize().unwrap()).unwrap();

        assert_eq!(serialized, serde_json::json!(["value1", "value2"]));
    }

    #[test]
    fn test_serialize_hashmap_basic() {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("key1".to_string(), "value1".to_string());
        hashmap.insert("key2".to_string(), "value2".to_string());

        let serialized: serde_json::Value =
            serde_json::from_str(&hashmap.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "key1": "value1",
                "key2": "value2"
            })
        );
    }

    #[test]
    fn test_serialize_hashmap_partial_recursion() {
        let hashmap = HashMap::from([
            (
                "value1".to_string(),
                MockSerializable {
                    value: "nested_value1".to_string(),
                },
            ),
            (
                "value2".to_string(),
                MockSerializable {
                    value: "nested_value2".to_string(),
                },
            ),
        ]);

        let serialized: serde_json::Value =
            serde_json::from_str(&hashmap.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "value1": "nested_value1",
                "value2": "nested_value2"
            })
        );
    }

    #[test]
    fn test_serialize_hashmap_recursion() {
        let mut hashmap: HashMap<String, HashMap<String, MockSerializable>> = HashMap::new();
        hashmap.insert(
            "key1".to_string(),
            HashMap::from([(
                "value1".to_string(),
                MockSerializable {
                    value: "nested_value1".to_string(),
                },
            )]),
        );
        hashmap.insert(
            "key2".to_string(),
            HashMap::from([(
                "value2".to_string(),
                MockSerializable {
                    value: "nested_value2".to_string(),
                },
            )]),
        );

        let serialized: serde_json::Value =
            serde_json::from_str(&hashmap.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "key1": {
                    "value1": "nested_value1"
                },
                "key2": {
                    "value2": "nested_value2"
                }
            })
        );
    }

    #[test]
    fn test_serialize_box() {
        let boxed_value = Box::new(MockSerializable {
            value: "boxed_value".to_string(),
        });
        assert_eq!(boxed_value.serialize(), Some("\"boxed_value\"".to_string()));
    }

    #[test]
    fn test_serialize_box_dyn() {
        let boxed_value: Box<dyn Serializable> = Box::new(MockSerializable {
            value: "boxed_dyn_value".to_string(),
        });
        assert_eq!(
            boxed_value.serialize(),
            Some("\"boxed_dyn_value\"".to_string())
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_serialize_chrono_fixed_offset() {
        use chrono::{DateTime, FixedOffset};

        let datetime =
            DateTime::<FixedOffset>::parse_from_rfc3339("1985-04-12T23:20:50.520Z").unwrap();

        assert_eq!(
            datetime.serialize(),
            Some("\"1985-04-12T23:20:50.520Z\"".to_string())
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_serialize_chrono_utc() {
        use chrono::{DateTime, FixedOffset, Utc};
        let datetime =
            DateTime::<FixedOffset>::parse_from_rfc3339("1985-04-12T23:20:50.520Z").unwrap();
        let datetime_utc = datetime.with_timezone(&Utc);

        assert_eq!(
            datetime_utc.serialize(),
            Some("\"1985-04-12T23:20:50.520Z\"".to_string())
        );
    }

    #[test]
    fn test_serialize_string_with_special_characters() {
        // Test case from issue #233 - special characters should be escaped
        let test_cases: Vec<(&str, &str)> = vec![
            ("simple", "\"simple\""),
            ("with\"quote", "\"with\\\"quote\""),
            ("with\\backslash", "\"with\\\\backslash\""),
            ("with\nnewline", "\"with\\nnewline\""),
            ("with\ttab", "\"with\\ttab\""),
            ("with\rcarriage", "\"with\\rcarriage\""),
            ("with/solidus", "\"with\\/solidus\""),
            (
                "a comma separated list of tags to filter by\nex: \"foo,bar\"",
                "\"a comma separated list of tags to filter by\\nex: \\\"foo,bar\\\"\"",
            ),
        ];

        for (input, expected) in test_cases {
            let serialized = input.to_string().serialize().unwrap();
            let result: serde_json::Value = serde_json::from_str(&serialized)
                .unwrap_or_else(|_| panic!("Failed to parse JSON for input: {}", input));
            let expected_result: serde_json::Value = serde_json::from_str(expected)
                .unwrap_or_else(|_| panic!("Failed to parse expected JSON: {}", expected));
            assert_eq!(result, expected_result, "Mismatch for input: {}", input);
        }
    }
}
