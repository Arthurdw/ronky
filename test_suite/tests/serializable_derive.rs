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
    assert!(
        json.get("is_deprecated").is_none(),
        "snake_case key should be omitted"
    );
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

    // Verify old field names are not present
    assert!(json.get("deprecated_since").is_none());
    assert!(json.get("deprecated_message").is_none());
    assert!(json.get("my_field_name").is_none());
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

    // Also test serialized nested metadata contents
    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(json["metadata"]["id"], "meta-id");
    assert_eq!(json["metadata"]["description"], "meta description");
    assert_eq!(json["metadata"]["isDeprecated"], false);
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

    // Also test serialized nullable output
    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(json["isNullable"], true);
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
    assert_eq!(json["isNullable"], true);
    // metadata field will be serialized as a nested object
    assert!(json["metadata"].is_object());
    // Check nested metadata fields and absence of unset entries
    assert_eq!(json["metadata"]["id"], "meta-id");
    assert!(json["metadata"].get("description").is_none());
    assert!(json["metadata"].get("isDeprecated").is_none());
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
fn test_disable_warnings_case_insensitive() {
    #[derive(SerializableDerive)]
    #[arri_disable(METADATA, Nullable)]
    struct CaseInsensitiveStruct {
        id: Option<String>,
        description: Option<String>,
    }

    let instance = CaseInsensitiveStruct {
        id: Some("test".to_string()),
        description: Some("description".to_string()),
    };

    // Should still serialize properly even with mixed case disable attributes
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
    let obj = json.as_object().unwrap();
    assert_eq!(obj.len(), 1);
    assert_eq!(obj.get("description").unwrap(), "only description");
    assert!(!obj.contains_key("id"));
}

#[test]
fn test_leading_underscore_handling() {
    #[derive(SerializableDerive)]
    #[arri_disable(metadata, nullable)]
    struct UnderscoreStruct {
        _private_field: Option<String>,
        _another_private: Option<String>,
        normal_field: Option<String>,
    }

    let instance = UnderscoreStruct {
        _private_field: Some("private".to_string()),
        _another_private: Some("another".to_string()),
        normal_field: Some("normal".to_string()),
    };

    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Check that leading underscores are preserved in field names
    assert_eq!(json["_privateField"], "private");
    assert_eq!(json["_anotherPrivate"], "another");
    assert_eq!(json["normalField"], "normal");
}

#[test]
fn test_none_field_omission() {
    #[derive(SerializableDerive)]
    #[arri_disable(metadata, nullable)]
    struct OptionalStruct {
        present_field: Option<String>,
        none_field: Option<String>,
        another_present: Option<i32>,
    }

    let instance = OptionalStruct {
        present_field: Some("value".to_string()),
        none_field: None,
        another_present: Some(42),
    };

    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();
    let obj = json.as_object().unwrap();

    // Check that None fields are completely omitted
    assert!(obj.contains_key("presentField"));
    assert!(obj.contains_key("anotherPresent"));
    assert!(!obj.contains_key("noneField"));

    assert_eq!(json["presentField"], "value");
    assert_eq!(json["anotherPresent"], 42);
}

#[test]
fn test_primitive_serializable_implementations() {
    use ronky::Serializable;

    // Test numeric types with JSON validation
    let i32_result = 42i32.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&i32_result).unwrap();
    assert_eq!(json, serde_json::json!(42));

    let u64_result = 1u64.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&u64_result).unwrap();
    assert_eq!(json, serde_json::json!(1));

    let f32_result = 3.5f32.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&f32_result).unwrap();
    assert_eq!(json.as_f64().unwrap(), 3.5);

    // Test string types
    assert_eq!("test".serialize().unwrap(), "\"test\"");
    assert_eq!("hello".to_string().serialize().unwrap(), "\"hello\"");

    // Test boolean
    assert_eq!(true.serialize().unwrap(), "true");
    assert_eq!(false.serialize().unwrap(), "false");
}

#[test]
fn test_nullable_vs_is_nullable_field_mapping() {
    #[derive(SerializableDerive)]
    #[arri_disable(metadata)]
    struct NullableFieldStruct {
        id: Option<String>,
        nullable: Option<bool>,
    }

    #[derive(SerializableDerive)]
    #[arri_disable(metadata)]
    struct IsNullableFieldStruct {
        id: Option<String>,
        is_nullable: Option<bool>,
    }

    let nullable_instance = NullableFieldStruct {
        id: Some("test".to_string()),
        nullable: Some(true),
    };

    let is_nullable_instance = IsNullableFieldStruct {
        id: Some("test".to_string()),
        is_nullable: Some(true),
    };

    let nullable_result = nullable_instance.serialize().unwrap();
    let is_nullable_result = is_nullable_instance.serialize().unwrap();

    let nullable_json: serde_json::Value = serde_json::from_str(&nullable_result).unwrap();
    let is_nullable_json: serde_json::Value = serde_json::from_str(&is_nullable_result).unwrap();

    // Both nullable and is_nullable fields should map to "isNullable" in JSON
    assert_eq!(nullable_json["isNullable"], true);
    assert_eq!(is_nullable_json["isNullable"], true);

    // Verify the snake_case field names are not present
    assert!(!nullable_json.as_object().unwrap().contains_key("nullable"));
    assert!(
        !is_nullable_json
            .as_object()
            .unwrap()
            .contains_key("is_nullable")
    );
}

#[test]
fn test_raw_identifier_handling() {
    #[derive(SerializableDerive)]
    #[arri_disable(metadata, nullable)]
    struct RawIdentifierStruct {
        r#type: Option<String>,
        r#ref: Option<String>,
        r#enum: Option<String>,
        normal_field: Option<String>,
    }

    let instance = RawIdentifierStruct {
        r#type: Some("object".to_string()),
        r#ref: Some("reference".to_string()),
        r#enum: Some("MyEnum".to_string()),
        normal_field: Some("normal".to_string()),
    };

    let result = instance.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Raw identifiers should be serialized without the r# prefix
    assert_eq!(json["type"], "object");
    assert_eq!(json["ref"], "reference");
    assert_eq!(json["enum"], "MyEnum");
    assert_eq!(json["normalField"], "normal");
}

#[test]
fn test_type_schema_setters() {
    use ronky::{MetadataSchema, TypeSchema, Types};

    let mut type_schema = TypeSchema::new(Types::String);

    // Test that TypeSchema now has set_metadata and set_nullable methods
    // (previously nullable was disabled, now it should work)
    let metadata = MetadataSchema {
        id: Some("test".to_string()),
        description: Some("A string type".to_string()),
        is_deprecated: None,
        deprecated_since: None,
        deprecated_message: None,
    };

    type_schema.set_metadata(metadata.clone());
    type_schema.set_nullable(true);

    let result = type_schema.serialize().unwrap();
    let json: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(json["type"], "string");
    assert_eq!(json["isNullable"], true);
    assert!(json["metadata"].is_object());
}
