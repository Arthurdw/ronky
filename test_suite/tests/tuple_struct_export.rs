use ronky::{ElementsSchema, Exportable, Exported, Serializable, TypeSchema, Types};

/// Test that a tuple struct wrapping String exports as just the String type.
/// The wrapper is transparent - no metadata from the wrapper is included.
#[test]
fn test_tuple_struct_string() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Email(String);

    let export = Email::export();
    // Tuple struct wrapper is transparent - exports exactly as the inner type
    let expected = TypeSchema::new(Types::String);

    assert!(export.is::<TypeSchema>());
    let export = export.downcast_ref::<TypeSchema>().unwrap();
    assert_eq!(*export, expected);
}

/// Test that a tuple struct wrapping i32 exports as just the Int32 type.
#[test]
fn test_tuple_struct_i32() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct UserId(i32);

    let export = UserId::export();
    // Tuple struct wrapper is transparent - exports exactly as the inner type
    let expected = TypeSchema::new(Types::Int32);

    assert!(export.is::<TypeSchema>());
    let export = export.downcast_ref::<TypeSchema>().unwrap();
    assert_eq!(*export, expected);
}

/// Test that doc comments on tuple structs are ignored since the wrapper is transparent.
/// "the client doesn't really benefit from actually knowing that the value was wrapped in a type"
#[test]
fn test_tuple_struct_doc_comments_ignored() {
    #[allow(dead_code)]
    #[derive(Exported)]
    /// This description is ignored - the wrapper is transparent
    struct ValidatedEmail(String);

    let export = ValidatedEmail::export();
    // Tuple struct wrapper is transparent - doc comments are not preserved
    let expected = TypeSchema::new(Types::String);

    assert!(export.is::<TypeSchema>());
    let export = export.downcast_ref::<TypeSchema>().unwrap();
    assert_eq!(*export, expected);
}

/// Test serialization produces just the inner type's schema.
#[test]
fn test_tuple_struct_serialization() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Counter(u64);

    let export = Counter::export();
    let serialized = export.serialize().unwrap();

    // Should serialize as just `{"type":"uint64"}` - no wrapper metadata
    assert_eq!(serialized, r#"{"type":"uint64"}"#);
}

/// Test that a tuple struct wrapping Vec exports as just the elements schema.
#[test]
fn test_tuple_struct_wrapping_vec() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Tags(Vec<String>);

    let export = Tags::export();
    // Tuple struct wrapper is transparent - exports exactly as Vec<String>
    let expected = ElementsSchema::new(Box::new(TypeSchema::new(Types::String)));

    assert!(export.is::<ElementsSchema>());
    let export = export.downcast_ref::<ElementsSchema>().unwrap();
    assert_eq!(*export, expected);
}

/// Test that nested tuple structs work - each layer is transparent.
#[test]
fn test_nested_tuple_structs() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Email(String);

    #[allow(dead_code)]
    #[derive(Exported)]
    struct ValidatedEmail(Email);

    let export = ValidatedEmail::export();
    // Both wrappers are transparent - exports as String
    let expected = TypeSchema::new(Types::String);

    assert!(export.is::<TypeSchema>());
    let export = export.downcast_ref::<TypeSchema>().unwrap();
    assert_eq!(*export, expected);
}
