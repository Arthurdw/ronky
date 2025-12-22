use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, RefSchema, Serializable, TypeSchema,
    Types,
};

#[test]
fn test_box_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct BoxExport {
        #[allow(clippy::box_collection)]
        value: Box<String>,
    }

    // This creates a stack overflow
    let export = BoxExport::export();
    let expected = {
        let mut schema = PropertiesSchema::new();

        schema.set_metadata(MetadataSchema::new().set_id("BoxExport").to_owned());
        schema.set_property("value", Box::new(TypeSchema::new(Types::String)));

        schema
    };

    assert_eq!(export.serialize(), expected.serialize());
}

#[test]
fn test_binary_tree_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct NumBinTree {
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
        value: i32,
    }

    let export = NumBinTree::export();
    let expected = {
        let mut schema = PropertiesSchema::new();

        schema.set_metadata(MetadataSchema::new().set_id("NumBinTree").to_owned());
        schema.set_optional_property("left", Box::new(RefSchema::new("NumBinTree")));
        schema.set_optional_property("right", Box::new(RefSchema::new("NumBinTree")));
        schema.set_property("value", Box::new(TypeSchema::new(Types::Int32)));

        schema
    };

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}
