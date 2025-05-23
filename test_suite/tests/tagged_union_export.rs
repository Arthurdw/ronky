use ronky::{
    EnumTransformation, Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable,
    TaggedUnionSchema,
};

#[test]
fn test_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = "uppercase", discriminator = "myDiscriminator")]
    enum Shape {
        Circle { radius: f64 },
    }

    let export = Shape::export();
    let mut expected = TaggedUnionSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Shape").to_owned());
    expected.set_discriminator("myDiscriminator");
    expected.add_mapping(
        "CIRCLE",
        Box::new({
            let mut props = PropertiesSchema::new();

            props.set_property(
                "radius",
                Box::new(ronky::TypeSchema::new(ronky::Types::Float64)),
            );

            props
        }),
    );
    expected.set_transforms(&[EnumTransformation::Uppercase]);

    assert!(export.is::<TaggedUnionSchema>());
    let export = export.downcast_ref::<TaggedUnionSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_export_unnamed() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = "uppercase", discriminator = "myDiscriminator")]
    enum Result {
        /// My example
        Ok(String),
        Nope(String),
    }

    let export = Result::export();
    let mut expected = TaggedUnionSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Result").to_owned());
    expected.set_discriminator("myDiscriminator");
    expected.add_mapping(
        "OK",
        Box::new({
            let mut props = PropertiesSchema::new();

            props.set_property(
                "value",
                Box::new(ronky::TypeSchema::new(ronky::Types::String)),
            );
            props.set_metadata(
                MetadataSchema::new()
                    .set_description("My example")
                    .to_owned(),
            );

            props
        }),
    );
    expected.add_mapping(
        "NOPE",
        Box::new({
            let mut props = PropertiesSchema::new();

            props.set_property(
                "value",
                Box::new(ronky::TypeSchema::new(ronky::Types::String)),
            );

            props
        }),
    );
    expected.set_transforms(&[EnumTransformation::Uppercase]);

    assert!(export.is::<TaggedUnionSchema>());
    let export = export.downcast_ref::<TaggedUnionSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_export_metadata() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum SampleUnion {
        /// This is a Circle
        Circle {
            /// The radius of the circle
            radius: f64,
        },
    }

    let export = SampleUnion::export();
    let mut expected = TaggedUnionSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("SampleUnion").to_owned());

    expected.add_mapping(
        "Circle",
        Box::new({
            let mut props = PropertiesSchema::new();

            props.set_property(
                "radius",
                Box::new({
                    let mut ty = ronky::TypeSchema::new(ronky::Types::Float64);
                    ty.set_metadata(
                        MetadataSchema::new()
                            .set_description("The radius of the circle")
                            .to_owned(),
                    );

                    ty
                }),
            );

            props.set_metadata(
                MetadataSchema::new()
                    .set_description("This is a Circle")
                    .to_owned(),
            );

            props
        }),
    );

    assert!(export.is::<TaggedUnionSchema>());
    let export = export.downcast_ref::<TaggedUnionSchema>().unwrap();
    assert_eq!(*export, expected);
}
