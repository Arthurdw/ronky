#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{Exportable, Exported, MetadataSchema, PropertiesSchema, TypeSchema, Types};

    #[test]
    fn test_export_option() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct TestStructOption {
            field1: Option<String>,
        }

        let export = TestStructOption::export();
        let mut expected = PropertiesSchema::new();
        expected
            .set_metadata(Box::new(
                MetadataSchema::new()
                    .set_id("TestStructOption".to_string())
                    .to_owned(),
            ))
            .set_optional_property("field1", Box::new(TypeSchema::new(Types::String)));

        assert_eq!(export, expected);
    }

    #[test]
    fn test_deprecated_struct() {
        #[deprecated(since = "1.0.0", note = "This struct is deprecated")]
        #[derive(Exported)]
        struct DeprecatedStruct {}

        let export = DeprecatedStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(Box::new(
            MetadataSchema::new()
                .set_id("DeprecatedStruct".to_string())
                .set_deprecated(true)
                .set_deprecated_since("1.0.0".to_string())
                .set_deprecated_message("This struct is deprecated".to_string())
                .to_owned(),
        ));

        assert_eq!(export, expected);
    }

    #[test]
    fn test_deprecated_struct_partial() {
        #[deprecated(note = "This struct is deprecated")]
        #[derive(Exported)]
        struct DeprecatedStructPartial {}

        let export = DeprecatedStructPartial::export();

        let mut expected = PropertiesSchema::new();
        expected.set_metadata(Box::new(
            MetadataSchema::new()
                .set_id("DeprecatedStructPartial".to_string())
                .set_deprecated(true)
                .set_deprecated_message("This struct is deprecated".to_string())
                .to_owned(),
        ));

        assert_eq!(export, expected);
    }

    #[test]
    fn test_documented_struct() {
        /// This is a struct to test if it can extrct the docs
        /// It should support multi-line comments aswell!
        #[derive(Exported)]
        struct DocumentedStruct {}

        let export = DocumentedStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(Box::new(
                MetadataSchema::new()
                    .set_id("DocumentedStruct".to_string())
                    .set_description("This is a struct to test if it can extrct the docs\nIt should support multi-line comments aswell!".to_string())
                    .to_owned(),
            ));

        assert_eq!(export, expected);
    }
}
