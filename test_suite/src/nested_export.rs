#[cfg(test)]
mod tests {
    use ronky::{Exportable, Exported, MetadataSchema, PropertiesSchema, TypeSchema, Types};

    #[test]
    fn test_export() {
        #[allow(dead_code)]
        // #[derive(Exported)]
        struct ChildExport {
            field: String,
        }

        #[allow(dead_code)]
        #[derive(Exported)]
        struct ParentExport {
            child: ChildExport,
        }

        let export = ParentExport::export();
        let mut expected = PropertiesSchema::new();
        expected
            .set_metadata(
                MetadataSchema::new()
                    .set_id("ParentExport".to_string())
                    .to_owned(),
            )
            .set_property(
                "child",
                Box::new({
                    let mut props = PropertiesSchema::new();
                    props
                        .set_property("field", Box::new(TypeSchema::new(Types::String)))
                        .set_metadata(
                            MetadataSchema::new()
                                .set_id("ChildExport".to_string())
                                .to_owned(),
                        );
                    props
                }),
            );

        assert_eq!(export, expected);
    }
}
