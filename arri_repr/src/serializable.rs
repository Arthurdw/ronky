// TODO: docs
// TODO: create a macro which automatically generates this implementation with a derive
use std::{any::type_name, collections::HashMap, fmt::Debug};

use downcast_rs::{Downcast, impl_downcast};

use crate::{MetadataSchema, serializer::Serializer};

pub trait Serializable: Downcast {
    fn serialize(&self) -> Option<String>;

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        panic!(
            "set_metadata is not implemented for this type!\n\
                This is a bug, please report it @ <https://github.com/Arthurdw/ronky/issues>\n\
                Type: {:?}\n\
                Serialized: {:?}\n\
                Metadata: {:?}",
            type_name::<Self>(),
            self.serialize(),
            metadata
        );
    }

    fn set_nullable(&mut self, nullable: bool) {
        panic!(
            "set_nullable is not implemented for this type!\n\
                This is a bug, please report it @ <https://github.com/Arthurdw/ronky/issues>\n\
                Type: {:?}\n\
                Serialized: {:?}\n\
                nullable: {:?}",
            type_name::<Self>(),
            self.serialize(),
            nullable
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
        format!("\"{}\"", self).into()
    }
}

impl Serializable for String {
    fn serialize(&self) -> Option<String> {
        format!("\"{}\"", self).into()
    }
}

impl Serializable for bool {
    fn serialize(&self) -> Option<String> {
        self.to_string().into()
    }
}

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
}
