#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{Exportable, Exported, MetadataSchema, PropertiesSchema, TypeSchema, Types};

    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[test]
    fn test_export() {
        let export = TestStruct::export();
        let mut expected = PropertiesSchema::new();
        expected
            .set_metadata(
                MetadataSchema::new()
                    .set_id("TestStruct".to_string())
                    .to_owned(),
            )
            .set_property("field1", Box::new(TypeSchema::new(Types::String)))
            .set_property("field2", Box::new(TypeSchema::new(Types::Int32)));

        assert_eq!(export, expected);
    }
}
