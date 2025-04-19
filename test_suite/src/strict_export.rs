#[cfg(test)]
mod tests {
    use ronky::{Exportable, Exported, PropertiesSchema};

    #[test]
    fn test_export() {
        #[derive(Exported)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let expected = PropertiesSchema::new();

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.is_strict, expected.is_strict);
    }

    #[test]
    fn test_export_strict() {
        #[derive(Exported)]
        #[arri(strict)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_strict(true);

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.is_strict, expected.is_strict);
    }

    #[test]
    fn test_export_strict_explicit_true() {
        #[derive(Exported)]
        #[arri(strict = true)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_strict(true);

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.is_strict, expected.is_strict);
    }

    #[test]
    fn test_export_strict_explicit_false() {
        #[derive(Exported)]
        #[arri(strict = false)]
        struct StrictStruct {}

        let export = StrictStruct::export();
        let mut expected = PropertiesSchema::new();
        expected.set_strict(false);

        assert!(export.is::<PropertiesSchema>());
        let export = export.downcast_ref::<PropertiesSchema>().unwrap();
        assert_eq!(export.is_strict, expected.is_strict);
    }
}
