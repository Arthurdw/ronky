use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
};

#[test]
fn test_export_option() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStructOption {
        field1: Option<String>,
    }

    let export = TestStructOption::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("TestStructOption".to_string())
            .to_owned(),
    );
    expected.set_optional_property("field1", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
#[allow(deprecated)]
fn test_deprecated_struct() {
    #[deprecated(since = "1.0.0", note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStruct {}

    let export = DeprecatedStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("DeprecatedStruct".to_string())
            .set_deprecated(true)
            .set_deprecated_since("1.0.0".to_string())
            .set_deprecated_message("This struct is deprecated".to_string())
            .to_owned(),
    );

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
#[allow(deprecated)]
fn test_deprecated_struct_partial() {
    #[deprecated(note = "This struct is deprecated")]
    #[derive(Exported)]
    struct DeprecatedStructPartial {}

    let export = DeprecatedStructPartial::export();

    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("DeprecatedStructPartial".to_string())
            .set_deprecated(true)
            .set_deprecated_message("This struct is deprecated".to_string())
            .to_owned(),
    );

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_documented_struct() {
    /// This is a struct to test if it can extrct the docs
    /// It should support multi-line comments aswell!
    #[derive(Exported)]
    struct DocumentedStruct {}

    let export = DocumentedStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
                MetadataSchema::new()
                    .set_id("DocumentedStruct".to_string())
                    .set_description("This is a struct to test if it can extrct the docs\nIt should support multi-line comments aswell!".to_string())
                    .to_owned(),
            );

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_metadata_struct_properties() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct MetadataStruct {
        #[deprecated(since = "1.0.0", note = "use field2 instead")]
        field1: String,

        /// Example docs
        field2: Option<String>,
    }

    let export = MetadataStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("MetadataStruct".to_string())
            .to_owned(),
    );
    expected.set_property(
        "field1",
        Box::new({
            use ronky::Serializable;
            let mut ty = TypeSchema::new(Types::String);
            ty.set_metadata(
                MetadataSchema::new()
                    .set_deprecated(true)
                    .set_deprecated_since("1.0.0".to_string())
                    .set_deprecated_message("use field2 instead".to_string())
                    .to_owned(),
            );
            ty
        }),
    );
    expected.set_optional_property(
        "field2",
        Box::new({
            use ronky::Serializable;
            let mut ty = TypeSchema::new(Types::String);
            ty.set_metadata(
                MetadataSchema::new()
                    .set_description("Example docs".to_string())
                    .to_owned(),
            );
            ty
        }),
    );

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_special_characters_in_doc_comments() {
    /// This is a description test
    /// Multiple lines should be supported without messing up JSON
    #[allow(dead_code)]
    #[derive(Exported)]
    struct MyStruct {
        /// a comma separated list of tags to filter by
        /// ex: "foo,bar"
        pub foo: String,
        pub bar: String,
    }

    let export = MyStruct::export();
    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();

    // Verify struct-level metadata with multi-line description
    let struct_desc = export
        .metadata
        .as_ref()
        .and_then(|m| m.description.as_ref())
        .expect("Struct should have description");
    assert!(struct_desc.contains("This is a description test"));
    assert!(struct_desc.contains("Multiple lines should be supported without messing up JSON"));

    // Verify that the serialized schema is valid JSON
    let serialized = export.serialize().expect("Export should be serializable");
    let json: serde_json::Value =
        serde_json::from_str(&serialized).expect("Serialized schema should be valid JSON");

    // Verify the struct description in JSON
    assert!(
        json["metadata"]["description"]
            .as_str()
            .expect("Struct description should be present")
            .contains("This is a description test")
    );

    // Verify field-level description with special characters in JSON
    let foo_desc = json["properties"]["foo"]["metadata"]["description"]
        .as_str()
        .expect("foo field description should be in JSON");

    // Verify that the multi-line description is properly formatted
    // The original has a newline between the two doc comment lines
    assert!(
        foo_desc.contains("comma separated list"),
        "multi-line content not found"
    );
    assert!(foo_desc.contains("foo,bar"), "quoted content not found");

    // Verify that the newline character is present (from joining doc comment lines)
    assert!(foo_desc.contains('\n'), "newline not found in description");

    // Verify escaping: the doc comment contains quotes around "foo,bar"
    // After JSON parsing, these appear as actual quote characters in the string
    assert!(foo_desc.contains('"'), "quote character not found");
}
