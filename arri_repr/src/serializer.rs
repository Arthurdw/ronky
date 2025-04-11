use crate::Serializable;

pub(crate) struct Serializer {
    pub(crate) out: String,
}

impl Serializer {
    pub(crate) fn builder() -> Self {
        Self {
            out: String::from("{"),
        }
    }

    pub(crate) fn set(&mut self, key: &str, value: &impl Serializable) -> &mut Self {
        if let Some(value) = value.serialize() {
            self.out.push_str(&format!("\"{}\":{},", key, value));
        }

        self
    }

    pub(crate) fn build(&mut self) -> String {
        if self.out.ends_with(',') {
            self.out.pop();
        }

        let out = self.out.clone();
        out + "}"
    }
}

impl From<Serializer> for String {
    fn from(mut serializer: Serializer) -> Self {
        serializer.build()
    }
}

#[cfg(test)]
mod tests {
    use super::Serializer;
    use crate::Serializable;

    struct MockSerializable<T> {
        value1: T,
        value2: T,
    }

    impl<T: Serializable> Serializable for MockSerializable<T> {
        fn serialize(&self) -> Option<String> {
            Serializer::builder()
                .set("value1", &self.value1)
                .set("value2", &self.value2)
                .build()
                .into()
        }
    }

    #[test]
    fn test_serializer_string() {
        let mock = MockSerializable {
            value1: "test_value1",
            value2: "test_value2",
        };

        let serialized: serde_json::Value =
            serde_json::from_str(&mock.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "value1": "test_value1",
                "value2": "test_value2"
            })
        );
    }

    #[test]
    fn test_serializer_empty() {
        assert_eq!(Serializer::builder().build(), "{}");
    }

    #[test]
    fn test_serializer_option() {
        let mock = MockSerializable {
            value1: Some("test_value1"),
            value2: Some("test_value2"),
        };

        let serialized: serde_json::Value =
            serde_json::from_str(&mock.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "value1": "test_value1",
                "value2": "test_value2"
            })
        );

        let mock_none = MockSerializable::<Option<String>> {
            value1: None,
            value2: None,
        };

        let serialized: serde_json::Value =
            serde_json::from_str(&mock_none.serialize().unwrap()).unwrap();

        assert_eq!(serialized, serde_json::json!({}));

        let mock_mixed = MockSerializable {
            value1: Some("test_value1"),
            value2: None,
        };

        let serialized: serde_json::Value =
            serde_json::from_str(&mock_mixed.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "value1": "test_value1",
            })
        );
    }
}
