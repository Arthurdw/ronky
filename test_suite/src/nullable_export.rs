#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use ronky::{Exportable, Exported, PropertiesSchema, TypeSchema, Types};

    #[test]
    fn test_export() {
        #[derive(Exported)]
        struct PropsStruct {
            value: String,
        }

        let export = PropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_property("value", Box::new(TypeSchema::new(Types::String)));

            prop
        };

        assert_eq!(export.properties, expected.properties);
    }

    #[test]
    fn test_nullable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable)]
            value: String,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_property(
                "value",
                Box::new({
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(true);
                    ty
                }),
            );

            prop
        };

        assert_eq!(export.properties, expected.properties);
    }

    #[test]
    fn test_nullable_explicit_enable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable = true)]
            value: String,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_property(
                "value",
                Box::new({
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(true);
                    ty
                }),
            );

            prop
        };

        assert_eq!(export.properties, expected.properties);
    }

    #[test]
    fn test_nullable_explicit_disable_export() {
        #[derive(Exported)]
        struct NullablePropsStruct {
            #[arri(nullable = false)]
            value: String,
        }

        let export = NullablePropsStruct::export();
        let expected = {
            let mut prop = PropertiesSchema::new();
            prop.set_property(
                "value",
                Box::new({
                    let mut ty = TypeSchema::new(Types::String);
                    ty.set_nullable(false);
                    ty
                }),
            );

            prop
        };

        assert_eq!(export.properties, expected.properties);
    }
}
