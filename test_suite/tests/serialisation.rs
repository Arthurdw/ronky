use std::str::FromStr;

use ronky::{
    Exportable, Exported, ExportedDeserialize, ExportedSerialize, MetadataSchema, PropertiesSchema,
    Serializable, TypeSchema, Types,
};
use serde::Serialize;

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

#[test]
fn test_serializing_string_escapes_special_characters() {
    #[derive(Exported, Serialize)]
    struct MyStruct {
        pub foo: String,
        pub bar: String,
    }

    let val = MyStruct {
        foo: String::from("hello\nworld"),
        bar: String::from("\"\\\x08\x0c\n\r\r\t"),
    };
    let json_str = val.to_json().expect("serialization succeeds");
    println!("RESULT: {:?}", &json_str);
    serde_json::Value::from_json(&json_str).expect("deserialization succeeds");
}

#[test]
fn test_serializing_metadata_escapes_special_characters() {
    #[allow(dead_code)]
    /// This is a description test
    /// Multiple lines should be supported without messing up JSON
    #[derive(Exported)]
    struct MyStruct {
        /// All special characters should also be supported \t \b " \f \r \u12345
        pub foo: String,
        pub bar: String,
    }

    let output = MyStruct::export().serialize();
    let expected_output = "{\"properties\":{\"foo\":{\"type\":\"string\",\"metadata\":{\"description\":\"All special characters should also be supported \\\\t \\\\b \\\" \\\\f \\\\r \\\\u12345\"}},\"bar\":{\"type\":\"string\"}},\"optionalProperties\":{},\"metadata\":{\"id\":\"MyStruct\",\"description\":\"This is a description test\\\\nMultiple lines should be supported without messing up JSON\"}}".to_string();
    assert_eq!(&output, &Some(expected_output));

    let _ = serde_json::Value::from_str(&output.unwrap()).expect("it should produce valid JSON");
}
