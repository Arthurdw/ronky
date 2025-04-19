#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use ronky::{Exportable, Exported, PropertiesSchema, TypeSchema, Types};

    #[test]
    fn test_export() {
        #[derive(Exported)]
        struct PropsStruct {
            value: Option<String>,
        }

        let export = PropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_optional_property("value", Box::new(TypeSchema::new(Types::String)));

            prop
        };

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.optional_properties, expected.optional_properties);
    }

    #[test]
    fn test_nullable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable)]
            value: Option<String>,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_optional_property(
                "value",
                Box::new({
                    use ronky::Serializable;
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(true);
                    ty
                }),
            );

            prop
        };

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.optional_properties, expected.optional_properties);
    }

    #[test]
    fn test_nullable_explicit_enable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable = true)]
            value: Option<String>,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_optional_property(
                "value",
                Box::new({
                    use ronky::Serializable;
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(true);
                    ty
                }),
            );

            prop
        };

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.optional_properties, expected.optional_properties);
    }

    #[test]
    fn test_nullable_explicit_disable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable = false)]
            value: Option<String>,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_optional_property(
                "value",
                Box::new({
                    use ronky::Serializable;
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(false);
                    ty
                }),
            );

            prop
        };

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.optional_properties, expected.optional_properties);
    }
}
