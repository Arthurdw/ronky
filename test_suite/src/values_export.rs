#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ronky::{
        Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
        ValuesSchema,
    };

    #[test]
    fn test_export() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct TestStruct {
            example: HashMap<String, bool>,
        }

        let export = TestStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("TestStruct".to_string())
                .to_owned(),
        );
        expected.set_property(
            "example",
            Box::new(ValuesSchema::new(Box::new(TypeSchema::new(Types::Boolean)))),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }
}
