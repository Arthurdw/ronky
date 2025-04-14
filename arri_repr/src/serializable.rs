// TODO: docs
use std::{collections::HashMap, fmt::Debug};

use crate::{MetadataSchema, serializer::Serializer};

// TODO: create a macro which automatically generates this implementation with a derive
pub trait Serializable {
    fn serialize(&self) -> Option<String>;

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        panic!(
            "set_metadata is not implemented for this type!\n
            This is a bug, please report it @ <https://github.com/Arthurdw/ronky/issues>\n
            Serialized: {:?}\n
            Metadata: {:?}",
            self.serialize(),
            metadata
        );
    }

    fn set_nullable(&mut self, nullable: bool) {
        panic!(
            "set_nullable is not implemented for this type!\n
            This is a bug, please report it @ <https://github.com/Arthurdw/ronky/issues>\n
            Serialized: {:?}\n
            nullable: {:?}",
            self.serialize(),
            nullable
        );
    }
}

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

impl Serializable for &str {
    fn serialize(&self) -> Option<String> {
        format!("\"{}\"", self).into()
    }
}

impl Serializable for String {
    fn serialize(&self) -> Option<String> {
        self.as_str().serialize()
    }
}

impl Serializable for bool {
    fn serialize(&self) -> Option<String> {
        self.to_string().into()
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
}

impl Serializable for Box<dyn Serializable> {
    fn serialize(&self) -> Option<String> {
        self.as_ref().serialize()
    }
}

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
}
