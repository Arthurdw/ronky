#[cfg(test)]
mod tests {
    use ronky::Exported;

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
}
