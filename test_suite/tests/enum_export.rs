use ronky::{EnumSchema, EnumTransformation, Exportable, Exported, MetadataSchema, Serializable};

#[test]
fn test_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum ExampleEnum {
        #[arri(rename = "firstVariant")]
        Variant1,
        Variant2,
    }

    let export = ExampleEnum::export();
    let mut expected = EnumSchema::new();
    expected.add_variant("firstVariant".to_string());
    expected.add_variant("Variant2".to_string());
    expected.set_metadata(MetadataSchema::new().set_id("ExampleEnum").to_owned());

    assert!(export.is::<EnumSchema>());
    let export = export.downcast_ref::<EnumSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_export_with_transformation() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = "uppercase")]
    enum ExampleEnum {
        #[arri(rename = "firstVariant")]
        Variant1,
        Variant2,
    }

    let export = ExampleEnum::export();
    let mut expected = EnumSchema::new();
    expected.add_variant("FIRSTVARIANT".to_string());
    expected.add_variant("VARIANT2".to_string());
    expected.set_metadata(MetadataSchema::new().set_id("ExampleEnum").to_owned());
    expected.set_transforms(&[EnumTransformation::Uppercase]);

    assert!(export.is::<EnumSchema>());
    let export = export.downcast_ref::<EnumSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_export_with_transformations() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = ["snake_case", "uppercase"])]
    enum ExampleEnum {
        MyVariant,
    }

    let export = ExampleEnum::export();
    let mut expected = EnumSchema::new();
    expected.add_variant("MY_VARIANT".to_string());
    expected.set_metadata(MetadataSchema::new().set_id("ExampleEnum").to_owned());
    expected.set_transforms(&[EnumTransformation::Snakecase, EnumTransformation::Uppercase]);

    assert!(export.is::<EnumSchema>());
    let export = export.downcast_ref::<EnumSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_export_with_transformations_multiline() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = "snake_case")]
    #[arri(transform = "uppercase")]
    enum ExampleEnum {
        MyVariant,
    }

    let export = ExampleEnum::export();
    let mut expected = EnumSchema::new();
    expected.add_variant("MY_VARIANT".to_string());
    expected.set_metadata(MetadataSchema::new().set_id("ExampleEnum").to_owned());
    expected.set_transforms(&[EnumTransformation::Snakecase, EnumTransformation::Uppercase]);

    assert!(export.is::<EnumSchema>());
    let export = export.downcast_ref::<EnumSchema>().unwrap();
    assert_eq!(*export, expected);
}
