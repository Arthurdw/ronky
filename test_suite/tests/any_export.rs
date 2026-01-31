//! Tests for the `any` feature flag and `Value` type.
//!
//! These tests verify that the `Value` type correctly exports to an empty Arri schema `{}`
//! and can serialize/deserialize any JSON value.

use ronky::{Exportable, Exported, PropertiesSchema, Serializable, Value};

// =============================================================================
// Schema Export Tests
// =============================================================================

#[test]
fn test_value_exports_to_empty_schema() {
    let export = Value::export();
    let serialized = export.serialize();

    // According to Arri spec, empty schema form is just `{}`
    assert_eq!(serialized, Some("{}".to_string()));
}

#[test]
fn test_value_in_struct_exports_correctly() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Config {
        name: String,
        /// Dynamic configuration that can be any JSON value
        data: Value,
    }

    let export = Config::export();
    assert!(export.is::<PropertiesSchema>());

    let serialized = export.serialize().expect("should serialize");

    // Parse to verify structure
    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).expect("should be valid JSON");

    // Check that 'data' field exists and has the empty schema form with metadata
    let properties = parsed.get("properties").expect("should have properties");
    let data_schema = properties.get("data").expect("should have data field");

    // The data field should have metadata with the description from the doc comment
    // but no type field (empty schema form)
    assert!(
        data_schema.get("type").is_none(),
        "should not have a type field (empty schema)"
    );
    assert!(
        data_schema.get("metadata").is_some(),
        "should have metadata from doc comment"
    );
    assert_eq!(
        data_schema["metadata"]["description"],
        "Dynamic configuration that can be any JSON value"
    );
}

#[test]
fn test_value_in_struct_without_docs_exports_to_empty_schema() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct SimpleConfig {
        name: String,
        data: Value,
    }

    let export = SimpleConfig::export();
    let serialized = export.serialize().expect("should serialize");

    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).expect("should be valid JSON");

    let properties = parsed.get("properties").expect("should have properties");
    let data_schema = properties.get("data").expect("should have data field");

    // Without doc comments, it should be a pure empty object
    assert_eq!(data_schema, &serde_json::json!({}));
}

#[test]
fn test_optional_value_exports_correctly() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct MaybeConfig {
        name: String,
        data: Option<Value>,
    }

    let export = MaybeConfig::export();
    let serialized = export.serialize().expect("should serialize");

    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).expect("should be valid JSON");

    // Optional Value should be in optionalProperties
    let optional_props = parsed
        .get("optionalProperties")
        .expect("should have optionalProperties");
    let data_schema = optional_props.get("data").expect("should have data field");

    // The data field should be an empty object (empty schema)
    assert_eq!(data_schema, &serde_json::json!({}));
}

#[test]
fn test_vec_of_value_exports_correctly() {
    #[allow(dead_code)]
    #[derive(Exported)]
    struct Items {
        values: Vec<Value>,
    }

    let export = Items::export();
    let serialized = export.serialize().expect("should serialize");

    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).expect("should be valid JSON");

    let properties = parsed.get("properties").expect("should have properties");
    let values_schema = properties.get("values").expect("should have values field");

    // Should be an elements schema with empty element type
    assert!(values_schema.get("elements").is_some());
    assert_eq!(
        values_schema.get("elements").unwrap(),
        &serde_json::json!({})
    );
}

#[test]
fn test_hashmap_with_value_exports_correctly() {
    use std::collections::HashMap;

    #[allow(dead_code)]
    #[derive(Exported)]
    struct DynamicFields {
        fields: HashMap<String, Value>,
    }

    let export = DynamicFields::export();
    let serialized = export.serialize().expect("should serialize");

    let parsed: serde_json::Value =
        serde_json::from_str(&serialized).expect("should be valid JSON");

    let properties = parsed.get("properties").expect("should have properties");
    let fields_schema = properties.get("fields").expect("should have fields field");

    // Should be a values schema with empty value type
    assert!(fields_schema.get("values").is_some());
    assert_eq!(fields_schema.get("values").unwrap(), &serde_json::json!({}));
}

// =============================================================================
// Value Construction Tests
// =============================================================================

#[test]
fn test_value_null_variant() {
    let value = Value::Null;
    assert_eq!(value, Value::Null);
}

#[test]
fn test_value_bool_variant() {
    let value_true = Value::Bool(true);
    let value_false = Value::Bool(false);

    assert_eq!(value_true, Value::Bool(true));
    assert_eq!(value_false, Value::Bool(false));
    assert_ne!(value_true, value_false);
}

#[test]
fn test_value_number_variants() {
    use ronky::NumberValue;

    let pos_int = Value::Number(NumberValue::PosInt(42));
    let neg_int = Value::Number(NumberValue::NegInt(-42));
    let float = Value::Number(NumberValue::Float(2.72));

    assert!(matches!(pos_int, Value::Number(NumberValue::PosInt(42))));
    assert!(matches!(neg_int, Value::Number(NumberValue::NegInt(-42))));
    assert!(
        matches!(float, Value::Number(NumberValue::Float(f)) if (f - 2.72).abs() < f64::EPSILON)
    );
}

#[test]
fn test_value_string_variant() {
    let value = Value::String("hello".to_string());
    assert_eq!(value, Value::String("hello".to_string()));
}

#[test]
fn test_value_array_variant() {
    use ronky::NumberValue;

    let value = Value::Array(vec![
        Value::Null,
        Value::Bool(true),
        Value::Number(NumberValue::PosInt(1)),
        Value::String("test".to_string()),
    ]);

    if let Value::Array(arr) = value {
        assert_eq!(arr.len(), 4);
        assert_eq!(arr[0], Value::Null);
        assert_eq!(arr[1], Value::Bool(true));
    } else {
        panic!("Expected Array variant");
    }
}

#[test]
fn test_value_object_variant() {
    use ronky::NumberValue;
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::String("test".to_string()));
    map.insert("count".to_string(), Value::Number(NumberValue::PosInt(5)));

    let value = Value::Object(map);

    if let Value::Object(obj) = value {
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("name"), Some(&Value::String("test".to_string())));
    } else {
        panic!("Expected Object variant");
    }
}

// =============================================================================
// JSON Serialization Tests (requires serialization feature)
// =============================================================================

#[test]
fn test_value_null_to_json() {
    use ronky::ExportedSerialize;

    let value = Value::Null;
    let json = value.to_json().expect("should serialize");
    assert_eq!(json, "null");
}

#[test]
fn test_value_bool_to_json() {
    use ronky::ExportedSerialize;

    assert_eq!(Value::Bool(true).to_json().unwrap(), "true");
    assert_eq!(Value::Bool(false).to_json().unwrap(), "false");
}

#[test]
fn test_value_number_to_json() {
    use ronky::{ExportedSerialize, NumberValue};

    assert_eq!(
        Value::Number(NumberValue::PosInt(42)).to_json().unwrap(),
        "42"
    );
    assert_eq!(
        Value::Number(NumberValue::NegInt(-42)).to_json().unwrap(),
        "-42"
    );

    let float_json = Value::Number(NumberValue::Float(2.72)).to_json().unwrap();
    assert!(float_json.starts_with("2.72"));
}

#[test]
fn test_value_string_to_json() {
    use ronky::ExportedSerialize;

    let value = Value::String("hello world".to_string());
    let json = value.to_json().expect("should serialize");
    assert_eq!(json, "\"hello world\"");
}

#[test]
fn test_value_string_escapes_special_chars() {
    use ronky::ExportedSerialize;

    let value = Value::String("hello\nworld\t\"test\"".to_string());
    let json = value.to_json().expect("should serialize");

    // Should properly escape special characters
    assert!(json.contains("\\n"));
    assert!(json.contains("\\t"));
    assert!(json.contains("\\\""));
}

#[test]
fn test_value_array_to_json() {
    use ronky::{ExportedSerialize, NumberValue};

    let value = Value::Array(vec![
        Value::Null,
        Value::Bool(true),
        Value::Number(NumberValue::PosInt(42)),
        Value::String("test".to_string()),
    ]);

    let json = value.to_json().expect("should serialize");
    assert_eq!(json, "[null,true,42,\"test\"]");
}

#[test]
fn test_value_object_to_json() {
    use ronky::{ExportedSerialize, NumberValue};
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::String("test".to_string()));
    map.insert("count".to_string(), Value::Number(NumberValue::PosInt(5)));

    let value = Value::Object(map);
    let json = value.to_json().expect("should serialize");

    // BTreeMap maintains sorted order
    assert_eq!(json, "{\"count\":5,\"name\":\"test\"}");
}

#[test]
fn test_value_nested_structure_to_json() {
    use ronky::{ExportedSerialize, NumberValue};
    use std::collections::BTreeMap;

    let mut inner = BTreeMap::new();
    inner.insert("x".to_string(), Value::Number(NumberValue::PosInt(1)));
    inner.insert("y".to_string(), Value::Number(NumberValue::PosInt(2)));

    let mut outer = BTreeMap::new();
    outer.insert("point".to_string(), Value::Object(inner));
    outer.insert(
        "tags".to_string(),
        Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]),
    );

    let value = Value::Object(outer);
    let json = value.to_json().expect("should serialize");

    // Verify it's valid JSON by parsing
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("should be valid JSON");
    assert!(parsed.is_object());
}

// =============================================================================
// JSON Deserialization Tests (requires serialization feature)
// =============================================================================

#[test]
fn test_value_null_from_json() {
    use ronky::ExportedDeserialize;

    let value = Value::from_json("null").expect("should deserialize");
    assert_eq!(value, Value::Null);
}

#[test]
fn test_value_bool_from_json() {
    use ronky::ExportedDeserialize;

    assert_eq!(Value::from_json("true").unwrap(), Value::Bool(true));
    assert_eq!(Value::from_json("false").unwrap(), Value::Bool(false));
}

#[test]
fn test_value_number_from_json() {
    use ronky::{ExportedDeserialize, NumberValue};

    let pos = Value::from_json("42").expect("should deserialize");
    assert!(matches!(pos, Value::Number(NumberValue::PosInt(42))));

    let neg = Value::from_json("-42").expect("should deserialize");
    assert!(matches!(neg, Value::Number(NumberValue::NegInt(-42))));

    let float = Value::from_json("2.72").expect("should deserialize");
    assert!(matches!(float, Value::Number(NumberValue::Float(f)) if (f - 2.72).abs() < 0.001));
}

#[test]
fn test_value_string_from_json() {
    use ronky::ExportedDeserialize;

    let value = Value::from_json("\"hello world\"").expect("should deserialize");
    assert_eq!(value, Value::String("hello world".to_string()));
}

#[test]
fn test_value_array_from_json() {
    use ronky::{ExportedDeserialize, NumberValue};

    let value = Value::from_json("[null, true, 42, \"test\"]").expect("should deserialize");

    if let Value::Array(arr) = value {
        assert_eq!(arr.len(), 4);
        assert_eq!(arr[0], Value::Null);
        assert_eq!(arr[1], Value::Bool(true));
        assert!(matches!(arr[2], Value::Number(NumberValue::PosInt(42))));
        assert_eq!(arr[3], Value::String("test".to_string()));
    } else {
        panic!("Expected Array variant");
    }
}

#[test]
fn test_value_object_from_json() {
    use ronky::{ExportedDeserialize, NumberValue};

    let value = Value::from_json("{\"name\": \"test\", \"count\": 5}").expect("should deserialize");

    if let Value::Object(obj) = value {
        assert_eq!(obj.len(), 2);
        assert_eq!(obj.get("name"), Some(&Value::String("test".to_string())));
        assert!(matches!(
            obj.get("count"),
            Some(Value::Number(NumberValue::PosInt(5)))
        ));
    } else {
        panic!("Expected Object variant");
    }
}

#[test]
fn test_value_roundtrip() {
    use ronky::{ExportedDeserialize, ExportedSerialize, NumberValue};
    use std::collections::BTreeMap;

    let mut inner = BTreeMap::new();
    inner.insert("enabled".to_string(), Value::Bool(true));
    inner.insert("count".to_string(), Value::Number(NumberValue::PosInt(100)));

    let original = Value::Object(inner);

    let json = original.to_json().expect("should serialize");
    let parsed = Value::from_json(&json).expect("should deserialize");

    assert_eq!(original, parsed);
}

#[test]
fn test_struct_with_value_serialization() {
    use ronky::{ExportedDeserialize, ExportedSerialize, NumberValue};
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Exported, Serialize, Deserialize, Debug, PartialEq)]
    struct Config {
        name: String,
        data: Value,
    }

    let mut data = BTreeMap::new();
    data.insert("key".to_string(), Value::String("value".to_string()));
    data.insert("num".to_string(), Value::Number(NumberValue::PosInt(42)));

    let config = Config {
        name: "test-config".to_string(),
        data: Value::Object(data),
    };

    let json = config.to_json().expect("should serialize");
    let parsed: Config = Config::from_json(&json).expect("should deserialize");

    assert_eq!(config, parsed);
}

// =============================================================================
// Conversion Tests
// =============================================================================

/// Helper function to convert serde_json::Value to ronky::Value
fn from_serde_json(v: serde_json::Value) -> Value {
    use ronky::NumberValue;

    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_u64() {
                Value::Number(NumberValue::PosInt(i))
            } else if let Some(i) = n.as_i64() {
                Value::Number(NumberValue::NegInt(i))
            } else if let Some(f) = n.as_f64() {
                Value::Number(NumberValue::Float(f))
            } else {
                Value::Number(NumberValue::Float(0.0))
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(from_serde_json).collect())
        }
        serde_json::Value::Object(obj) => Value::Object(
            obj.into_iter()
                .map(|(k, v)| (k, from_serde_json(v)))
                .collect(),
        ),
    }
}

/// Helper function to convert ronky::Value to serde_json::Value
fn to_serde_json(v: Value) -> serde_json::Value {
    use ronky::NumberValue;

    match v {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Number(n) => match n {
            NumberValue::PosInt(i) => serde_json::Value::Number(i.into()),
            NumberValue::NegInt(i) => serde_json::Value::Number(i.into()),
            NumberValue::Float(f) => serde_json::Number::from_f64(f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
        },
        Value::String(s) => serde_json::Value::String(s),
        Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(to_serde_json).collect()),
        Value::Object(obj) => serde_json::Value::Object(
            obj.into_iter()
                .map(|(k, v)| (k, to_serde_json(v)))
                .collect(),
        ),
    }
}

#[test]
fn test_value_from_serde_json_value() {
    let json_value = serde_json::json!({
        "name": "test",
        "count": 42,
        "enabled": true,
        "tags": ["a", "b", "c"],
        "nested": {
            "x": 1,
            "y": 2
        },
        "nothing": null
    });

    let value: Value = from_serde_json(json_value.clone());

    // Convert back and compare
    let back: serde_json::Value = to_serde_json(value);
    assert_eq!(json_value, back);
}

#[test]
fn test_value_into_serde_json_value() {
    use ronky::NumberValue;
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert("name".to_string(), Value::String("test".to_string()));
    map.insert("count".to_string(), Value::Number(NumberValue::PosInt(42)));

    let value = Value::Object(map);
    let json_value: serde_json::Value = to_serde_json(value);

    assert_eq!(json_value["name"], "test");
    assert_eq!(json_value["count"], 42);
}
