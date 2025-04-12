#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{
        Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
    };

    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        string: String,
        // TODO: add support for string_ref
        // string_ref: &'static str,
        boolean: bool,
        // TODO: figure out how to do timestamps
        // timestamp: DateTime<Utc>,
        float32: f32,
        float64: f64,
        int8: i8,
        uint8: u8,
        int16: i16,
        uint16: u16,
        int32: i32,
        uint32: u32,
        int64: i64,
        uint64: u64,
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
            .set_property("string", Box::new(TypeSchema::new(Types::String)))
            // .set_property("string_ref", Box::new(TypeSchema::new(Types::String)))
            .set_property("boolean", Box::new(TypeSchema::new(Types::Boolean)))
            .set_property("float32", Box::new(TypeSchema::new(Types::Float32)))
            .set_property("float64", Box::new(TypeSchema::new(Types::Float64)))
            .set_property("int8", Box::new(TypeSchema::new(Types::Int8)))
            .set_property("uint8", Box::new(TypeSchema::new(Types::Uint8)))
            .set_property("int16", Box::new(TypeSchema::new(Types::Int16)))
            .set_property("uint16", Box::new(TypeSchema::new(Types::Uint16)))
            .set_property("int32", Box::new(TypeSchema::new(Types::Int32)))
            .set_property("uint32", Box::new(TypeSchema::new(Types::Uint32)))
            .set_property("int64", Box::new(TypeSchema::new(Types::Int64)))
            .set_property("uint64", Box::new(TypeSchema::new(Types::Uint64)))
            // .set_property("timestamp", Box::new(TypeSchema::new(Types::Timestamp)))
            .serialize();

        assert!(serialized.is_some());
        assert!(expected.is_some());

        let json: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(&expected.unwrap()).unwrap();

        assert_eq!(json, expected_json,);
    }
}
