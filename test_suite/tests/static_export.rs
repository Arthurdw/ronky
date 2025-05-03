use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
};

#[test]
fn test_static_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Bar {
        a: String,
    }

    #[allow(dead_code)]
    #[derive(Exported)]
    struct Foo {
        b: Bar,
    }

    let export = Foo::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Foo".to_string()).to_owned());
    expected.set_property(
        "b",
        Box::new({
            let mut bar = PropertiesSchema::new();
            bar.set_metadata(MetadataSchema::new().set_id("Bar".to_string()).to_owned());
            bar.set_property("a", Box::new(TypeSchema::new(Types::String)));
            bar
        }),
    );

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}
