#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{
        Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
    };

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

        let expected = PropertiesSchema::new()
            .set_metadata(Box::new(
                MetadataSchema::new()
                    .set_id("TestStruct".to_string())
                    .to_owned(),
            ))
            .set_property("field1", Box::new(TypeSchema::new(Types::String)))
            .set_property("field2", Box::new(TypeSchema::new(Types::Int32)))
            .serialize();

        assert!(serialized.is_some());
        assert!(expected.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(&expected.unwrap()).unwrap();

        assert_eq!(json, expected_json,);
    }

    #[deprecated(since = "1.0.0", note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStruct {}

    #[test]
    fn test_deprecated_struct() {
        let export = DeprecatedStruct::export();
        let serialized = export.serialize();

        let expected = PropertiesSchema::new()
            .set_metadata(Box::new(
                MetadataSchema::new()
                    .set_id("DeprecatedStruct".to_string())
                    .set_deprecated(true)
                    .set_deprecated_since("1.0.0".to_string())
                    .set_deprecated_message("This struct is deprecated".to_string())
                    .to_owned(),
            ))
            .serialize();

        assert!(serialized.is_some());
        assert!(expected.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(&expected.unwrap()).unwrap();
        assert_eq!(json, expected_json);
    }

    #[deprecated(note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStructPartial {}

    #[test]
    fn test_deprecated_struct_partial() {
        let export = DeprecatedStructPartial::export();
        let serialized = export.serialize();

        let expected = PropertiesSchema::new()
            .set_metadata(Box::new(
                MetadataSchema::new()
                    .set_id("DeprecatedStructPartial".to_string())
                    .set_deprecated(true)
                    .set_deprecated_message("This struct is deprecated".to_string())
                    .to_owned(),
            ))
            .serialize();

        assert!(serialized.is_some());
        assert!(expected.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(&expected.unwrap()).unwrap();

        assert_eq!(json, expected_json,);
    }
}
