#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{Exportable, Exported, Serializable};

    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_export() {
        let export = TestStruct::export();
        let serialized = export.serialize();

        assert!(serialized.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "metadata": {
                    "id": "TestStruct",
                    "isDeprecated": false
                },
                "properties": {
                    "field1": {
                        "type": "string"
                    },
                    "field2": {
                        "type": "int32"
                    }
                },
                "optionalProperties": {}
            })
        );
    }

    #[deprecated(since = "1.0.0", note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStruct {}

    #[test]
    fn test_deprecated_struct() {
        let export = DeprecatedStruct::export();
        let serialized = export.serialize();

        assert!(serialized.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "metadata": {
                    "id": "DeprecatedStruct",
                    "isDeprecated": true,
                    "deprecatedSince": "1.0.0",
                    "deprecatedNote": "This struct is deprecated"
                },
                "properties": {},
                "optionalProperties": {}
            })
        );
    }

    #[deprecated(note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStructPartial {}

    #[test]
    fn test_deprecated_struct_partial() {
        let export = DeprecatedStructPartial::export();
        let serialized = export.serialize();

        assert!(serialized.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "metadata": {
                    "id": "DeprecatedStructPartial",
                    "isDeprecated": true,
                    "deprecatedNote": "This struct is deprecated"
                },
                "properties": {},
                "optionalProperties": {}
            })
        );
    }
}
