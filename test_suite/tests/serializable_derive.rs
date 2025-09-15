use ronky::{MetadataSchema, Serializable, SerializableDerive};

#[test]
fn test_basic_serializable_derive() {
    #[derive(SerializableDerive)]
    struct BasicStruct {
        id: Option<String>,
        description: Option<String>,
        is_deprecated: Option<bool>,
    }

    let instance = BasicStruct {
        id: Some("test-id".to_string()),
        description: Some("test description".to_string()),
        is_deprecated: Some(true),
    };

    let result = instance.serialize().unwrap();

    // Parse the JSON to verify it's valid and contains expected fields
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(json["id"], "test-id");
    assert_eq!(json["description"], "test description");
    assert_eq!(json["isDeprecated"], true);
}

#[test]
fn test_field_name_transformations() {
    #[derive(SerializableDerive)]
    struct TransformStruct {
        deprecated_since: Option<String>,
        deprecated_message: Option<String>,
        my_field_name: Option<String>,
    }

    let instance = TransformStruct {
        deprecated_since: Some("1.0.0".to_string()),
        deprecated_message: Some("Use new API".to_string()),
        my_field_name: Some("value".to_string()),
    };

    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Check field name transformations
    assert_eq!(json["deprecatedSince"], "1.0.0");
    assert_eq!(json["deprecatedNote"], "Use new API");
    assert_eq!(json["myFieldName"], "value");
}

#[test]
fn test_metadata_field_detection() {
    #[derive(SerializableDerive)]
    struct MetadataStruct {
        id: Option<String>,
        metadata: Option<MetadataSchema>,
    }

    let mut instance = MetadataStruct {
        id: Some("test".to_string()),
        metadata: None,
    };

    // Test that set_metadata is implemented
    let test_metadata = MetadataSchema {
        id: Some("meta-id".to_string()),
        description: Some("meta description".to_string()),
        is_deprecated: Some(false),
        deprecated_since: None,
        deprecated_message: None,
    };

    instance.set_metadata(test_metadata.clone());

    assert_eq!(instance.metadata, Some(test_metadata));
}

#[test]
fn test_nullable_field_detection() {
    #[derive(SerializableDerive)]
    struct NullableStruct {
        id: Option<String>,
        nullable: Option<bool>,
    }

    let mut instance = NullableStruct {
        id: Some("test".to_string()),
        nullable: None,
    };

    // Test that set_nullable is implemented
    instance.set_nullable(true);

    assert_eq!(instance.nullable, Some(true));
}

#[test]
fn test_combined_metadata_and_nullable() {
    #[derive(SerializableDerive)]
    struct CombinedStruct {
        id: Option<String>,
        description: Option<String>,
        metadata: Option<MetadataSchema>,
        nullable: Option<bool>,
    }

    let mut instance = CombinedStruct {
        id: Some("test".to_string()),
        description: Some("description".to_string()),
        metadata: None,
        nullable: None,
    };

    // Test both methods work
    let test_metadata = MetadataSchema {
        id: Some("meta-id".to_string()),
        description: None,
        is_deprecated: None,
        deprecated_since: None,
        deprecated_message: None,
    };

    instance.set_metadata(test_metadata.clone());
    instance.set_nullable(true);

    assert_eq!(instance.metadata, Some(test_metadata));
    assert_eq!(instance.nullable, Some(true));

    // Test serialization includes all fields
    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(json["id"], "test");
    assert_eq!(json["description"], "description");
    assert_eq!(json["nullable"], true);
    // metadata field will be serialized as a nested object
    assert!(json["metadata"].is_object());
}

#[test]
fn test_disable_warnings() {
    #[derive(SerializableDerive)]
    #[arri_disable(metadata, nullable)]
    struct DisabledWarningsStruct {
        id: Option<String>,
        description: Option<String>,
    }

    let instance = DisabledWarningsStruct {
        id: Some("test".to_string()),
        description: Some("description".to_string()),
    };

    // Should still serialize properly
    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(json["id"], "test");
    assert_eq!(json["description"], "description");
}

#[test]
fn test_empty_optional_fields() {
    #[derive(SerializableDerive)]
    struct EmptyStruct {
        id: Option<String>,
        description: Option<String>,
        count: Option<i32>,
    }

    let instance = EmptyStruct {
        id: None,
        description: Some("only description".to_string()),
        count: None,
    };

    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Only non-None fields should be included
    assert_eq!(json.as_object().unwrap().len(), 1);
    assert_eq!(json["description"], "only description");
    assert!(json["id"].is_null() || !json.as_object().unwrap().contains_key("id"));
}
