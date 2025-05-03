use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
};

#[test]
fn test_rename() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        #[arri(rename = "myFieldName")]
        field1: String,
    }

    let export = TestStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("TestStruct".to_string())
            .to_owned(),
    );
    expected.set_property("myFieldName", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}
