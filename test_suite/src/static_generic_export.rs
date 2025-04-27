#[cfg(test)]
mod tests {
    use ronky::{
        ElementsSchema, Exportable, Exported, MetadataSchema, PropertiesSchema, RefSchema,
        Serializable, TypeSchema, Types,
    };

    #[test]
    fn test_type_export() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct Foo<T: Exportable> {
            a: T,
        }

        type Bar = Foo<String>;

        let export = Bar::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("FooString".to_string())
                .to_owned(),
        );
        expected.set_property("a", Box::new(TypeSchema::new(Types::String)));

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }

    #[test]
    fn test_nested_export() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct InnerExample<T: Exportable> {
            b: T,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct OuterExample<T: Exportable> {
            a: T,
        }

        type MyType = OuterExample<InnerExample<String>>;

        let export = MyType::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("OuterExampleInnerExampleString".to_string())
                .to_owned(),
        );
        expected.set_property(
            "a",
            Box::new({
                let mut inner = PropertiesSchema::new();
                inner.set_metadata(
                    MetadataSchema::new()
                        .set_id("InnerExampleString".to_string())
                        .to_owned(),
                );
                inner.set_property("b", Box::new(TypeSchema::new(Types::String)));

                inner
            }),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }

    #[test]
    fn test_ref_generic_nested() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct Foo {
            nested: Option<Box<Bar<Self>>>,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct Bar<T: Exportable> {
            of: T,
        }

        let export = Bar::<Foo>::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("BarFoo".to_string())
                .to_owned(),
        );
        expected.set_property(
            "of",
            Box::new({
                let mut foo = PropertiesSchema::new();
                foo.set_metadata(MetadataSchema::new().set_id("Foo".to_string()).to_owned());
                foo.set_optional_property("nested", Box::new(RefSchema::new("BarFoo")));
                foo
            }),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }

    #[test]
    fn test_ref_generic_less_nested() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct Foo {
            less_nested: Option<Box<Self>>,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct Bar<T: Exportable> {
            of: T,
        }

        let export = Bar::<Foo>::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("BarFoo".to_string())
                .to_owned(),
        );
        expected.set_property(
            "of",
            Box::new({
                let mut foo = PropertiesSchema::new();
                foo.set_metadata(MetadataSchema::new().set_id("Foo".to_string()).to_owned());
                foo.set_optional_property("less_nested", Box::new(RefSchema::new("Foo")));
                foo
            }),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }

    #[test]
    fn test_ref_generic_more_nested() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct ActuallySomething<T: Exportable> {
            value: T,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct Foo {
            more_nested: Option<ActuallySomething<Box<Self>>>,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct Bar<T: Exportable> {
            of: T,
        }

        let export = Bar::<Foo>::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("BarFoo".to_string())
                .to_owned(),
        );
        expected.set_property(
            "of",
            Box::new({
                let mut foo = PropertiesSchema::new();
                foo.set_metadata(MetadataSchema::new().set_id("Foo".to_string()).to_owned());
                foo.set_optional_property(
                    "more_nested",
                    Box::new({
                        let mut nested = PropertiesSchema::new();
                        nested.set_metadata(
                            MetadataSchema::new()
                                .set_id("ActuallySomethingFoo".to_string())
                                .to_owned(),
                        );
                        nested.set_property("value", Box::new(RefSchema::new("Foo")));

                        nested
                    }),
                );

                foo
            }),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }

    #[test]
    fn test_ref_generic_metadata_id_present() {
        #[allow(dead_code)]
        #[derive(Exported)]
        struct Human {
            /// Example
            friends: Vec<Human>,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct About<T: Exportable> {
            /// More example
            of: T,
        }

        let export = About::<Human>::export();
        let mut expected = PropertiesSchema::new();
        expected.set_metadata(
            MetadataSchema::new()
                .set_id("AboutHuman".to_string())
                .to_owned(),
        );
        expected.set_property(
            "of",
            Box::new({
                let mut foo = PropertiesSchema::new();
                foo.set_metadata(
                    MetadataSchema::new()
                        .set_id("Human")
                        .set_description("More example")
                        .to_owned(),
                );
                foo.set_property(
                    "friends",
                    Box::new({
                        let mut el = ElementsSchema::new(Box::new(RefSchema::new("Human")));
                        el.set_metadata(
                            MetadataSchema::new().set_description("Example").to_owned(),
                        );
                        el
                    }),
                );
                foo
            }),
        );

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(*export, expected);
    }
}
