#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use ronky::{Exportable, Exported, PropertiesSchema};

    #[test]
    fn test_export() {
        #[derive(Exported)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let expected = PropertiesSchema::new();

        assert_eq!(export.strict, expected.strict);
    }

    #[test]
    fn test_export_strict() {
        #[derive(Exported)]
        #[arri(strict)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_strict(true);

        assert_eq!(export.strict, expected.strict);
    }
}
