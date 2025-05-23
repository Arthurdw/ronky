use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
};

#[test]
fn test_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    let export = TestStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("TestStruct".to_string())
            .to_owned(),
    );
    expected.set_property("field1", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("field2", Box::new(TypeSchema::new(Types::Int32)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}
