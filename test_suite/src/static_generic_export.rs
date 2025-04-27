#[cfg(test)]
mod tests {
    use ronky::{
        Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
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
}
