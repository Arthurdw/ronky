#[cfg(test)]
mod tests {
    use ronky::{Exportable, Exported, MetadataSchema, PropertiesSchema, TypeSchema, Types};

    #[test]
    fn test_box_export() {
        #[derive(Exported)]
        struct BoxExport {
            value: Box<String>,
        }

        // This creates a stack overflow
        let export = BoxExport::export();
        let expected = {
            let mut schema = PropertiesSchema::new();

            schema
                .set_metadata(MetadataSchema::new().set_id("BoxExport").to_owned())
                .set_property("value", Box::new(TypeSchema::new(Types::String)));

            schema
        };

        assert_eq!(export, expected,);
    }

    #[test]
    fn test_binary_tree_export() {
        #[derive(Exported)]
        struct NumBinTree {
            left: Option<Box<NumBinTree>>,
            right: Option<Box<NumBinTree>>,
            value: i32,
        }

        let export = NumBinTree::export();
        let expected = {
            let mut schema = PropertiesSchema::new();

            schema.set_metadata(MetadataSchema::new().set_id("NumBinTree").to_owned());

            schema
        };

        assert_eq!(export, expected,);
    }
}
