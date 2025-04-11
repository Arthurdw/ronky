// TODO: docs
use std::{collections::HashMap, fmt::Debug};

use crate::serializer::Serializer;

pub trait Serializable {
    fn serialize(&self) -> Option<String>;
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
        Some(format!("\"{}\"", self))
    }
}

impl Serializable for String {
    fn serialize(&self) -> Option<String> {
        Some(format!("\"{}\"", self))
    }
}

impl Serializable for bool {
    fn serialize(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Option<String> {
        let serialized_elements: Vec<String> = self.iter().filter_map(|e| e.serialize()).collect();
        Some(format!("[{}]", serialized_elements.join(",")))
    }
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self) -> Option<String> {
        match self {
            Some(value) => value.serialize(),
            None => None,
        }
    }
}

impl<T: Serializable + Clone> Serializable for HashMap<String, T> {
    fn serialize(&self) -> Option<String> {
        let mut builder = Serializer::builder();

        for (key, value) in self.iter() {
            builder.set(key, value.clone());
        }

        Some(builder.build())

        // Some(
        //     self.iter()
        //         .filter_map(|(k, v)| {
        //             let serialized_value = v.serialize()?;
        //             Some(format!("\"{}\":{}", k, serialized_value))
        //         })
        //         .collect::<Vec<String>>()
        //         .as_slice()
        //         .join(","),
        // )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
