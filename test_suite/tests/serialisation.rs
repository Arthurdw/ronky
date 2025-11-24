use ronky::{
    Exportable, Exported, MetadataSchema, PropertiesSchema, Serializable, TypeSchema, Types,
};

#[test]
fn test_rename() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct TestStruct {
        #[arri(rename = "myFieldName")]
        field1: String,
    }

    let export = TestStruct::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(
        MetadataSchema::new()
            .set_id("TestStruct".to_string())
            .to_owned(),
    );
    expected.set_property("myFieldName", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_camel_case() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "camelCase")]
    struct Book {
        id: String,
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("authorName", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("publishDate", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_pascal_case() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "PascalCase")]
    struct Book {
        id: String,
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("Id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("AuthorName", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("PublishDate", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_snake_case() {
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    #[derive(Exported)]
    #[arri(rename_all = "snake_case")]
    struct Book {
        id: String,
        authorName: String,
        publishDate: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("author_name", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("publish_date", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_screaming_snake_case() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "SCREAMING_SNAKE_CASE")]
    struct Book {
        id: String,
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("ID", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("AUTHOR_NAME", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("PUBLISH_DATE", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_kebab_case() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "kebab-case")]
    struct Book {
        id: String,
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("author-name", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("publish-date", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_screaming_kebab_case() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "SCREAMING-KEBAB-CASE")]
    struct Book {
        id: String,
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("ID", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("AUTHOR-NAME", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("PUBLISH-DATE", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_rename_all_with_explicit_rename() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(rename_all = "camelCase")]
    struct Book {
        id: String,
        #[arri(rename = "customAuthorName")]
        author_name: String,
        publish_date: String,
    }

    let export = Book::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Book".to_string()).to_owned());
    expected.set_property("id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("customAuthorName", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("publishDate", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}

#[test]
fn test_raw_identifier_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Event {
        id: String,
        r#type: String,
    }

    let export = Event::export();
    let mut expected = PropertiesSchema::new();
    expected.set_metadata(MetadataSchema::new().set_id("Event".to_string()).to_owned());
    expected.set_property("id", Box::new(TypeSchema::new(Types::String)));
    expected.set_property("type", Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<PropertiesSchema>());
    let export = export.downcast_ref::<PropertiesSchema>().unwrap();
    assert_eq!(*export, expected);
}
