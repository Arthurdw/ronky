use ronky::{Exportable, Exported, UnionSchema};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

fn schema_of<T: Exportable>() -> Value {
    serde_json::from_str(&T::export().serialize().unwrap()).unwrap()
}

/// A minimal validator for the subset of Arri type definitions used in these tests.
///
/// `root` is used to resolve `ref` schemas. Properties are validated strictly so that any
/// mismatch between serde's output and the exported schema is caught.
fn validate(root: &Value, schema: &Value, value: &Value) -> bool {
    validate_with_extra(root, schema, value, None)
}

fn validate_with_extra(root: &Value, schema: &Value, value: &Value, extra: Option<&str>) -> bool {
    if value.is_null() && schema["isNullable"] == json!(true) {
        return true;
    }

    if schema.get("ref").is_some() {
        return validate(root, root, value);
    }

    if let Some(ty) = schema["type"].as_str() {
        return match ty {
            "string" => value.is_string(),
            "boolean" => value.is_boolean(),
            "float32" | "float64" => value.is_number(),
            "int8" | "int16" | "int32" => value.is_i64(),
            "uint8" | "uint16" | "uint32" => value.is_u64(),
            other => panic!("unsupported type in test validator: {other}"),
        };
    }

    if let Some(variants) = schema["enum"].as_array() {
        return variants.contains(value);
    }

    if let Some(elements) = schema.get("elements") {
        return value
            .as_array()
            .is_some_and(|items| items.iter().all(|item| validate(root, elements, item)));
    }

    if let Some(props) = schema["properties"].as_object() {
        let Some(obj) = value.as_object() else {
            return false;
        };
        let optional = schema["optionalProperties"].as_object();

        let required_ok = props
            .iter()
            .all(|(key, s)| obj.get(key).is_some_and(|v| validate(root, s, v)));
        let rest_ok = obj.iter().all(|(key, v)| {
            props.contains_key(key)
                || Some(key.as_str()) == extra
                || optional
                    .and_then(|o| o.get(key))
                    .is_some_and(|s| validate(root, s, v))
        });

        return required_ok && rest_ok;
    }

    if let Some(variants) = schema["union"].as_object() {
        let Some(obj) = value.as_object() else {
            return false;
        };

        return match (schema["tagKey"].as_str(), schema["valueKey"].as_str()) {
            (None, None) => {
                obj.len() == 1 && {
                    let (tag, inner) = obj.iter().next().unwrap();
                    variants.get(tag).is_some_and(|s| validate(root, s, inner))
                }
            }
            (Some(tag_key), None) => obj
                .get(tag_key)
                .and_then(Value::as_str)
                .and_then(|tag| variants.get(tag))
                .is_some_and(|s| validate_with_extra(root, s, value, Some(tag_key))),
            (Some(tag_key), Some(value_key)) => {
                obj.len() == 2
                    && obj
                        .get(tag_key)
                        .and_then(Value::as_str)
                        .and_then(|tag| variants.get(tag))
                        .zip(obj.get(value_key))
                        .is_some_and(|(s, inner)| validate(root, s, inner))
            }
            (None, Some(_)) => panic!("valueKey without tagKey"),
        };
    }

    // Empty schema form
    schema
        .as_object()
        .is_some_and(|o| o.keys().all(|k| k == "metadata" || k == "isNullable"))
}

fn assert_valid<T: Exportable + Serialize>(value: &T) {
    let schema = schema_of::<T>();
    let json = serde_json::to_value(value).unwrap();
    assert!(
        validate(&schema, &schema, &json),
        "serde output does not match the exported schema\nschema: {schema:#}\nvalue: {json:#}"
    );
}

#[test]
fn test_externally_tagged_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    /// A shape
    enum Shape {
        /// A circle
        Circle {
            radius: f64,
        },
        Rectangle {
            width: f64,
            height: f64,
        },
        /// A label
        Label(String),
        Maybe(Option<u32>),
    }

    let export = Shape::export();
    assert!(export.is::<UnionSchema>());

    assert_eq!(
        schema_of::<Shape>(),
        json!({
            "union": {
                "Circle": {
                    "properties": { "radius": { "type": "float64" } },
                    "optionalProperties": {},
                    "metadata": { "description": "A circle" }
                },
                "Rectangle": {
                    "properties": {
                        "width": { "type": "float64" },
                        "height": { "type": "float64" }
                    },
                    "optionalProperties": {}
                },
                "Label": { "type": "string", "metadata": { "description": "A label" } },
                "Maybe": { "type": "uint32", "isNullable": true }
            },
            "metadata": { "id": "Shape", "description": "A shape" }
        })
    );
}

#[test]
fn test_externally_tagged_transforms_and_renames() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(transform = ["snake_case", "uppercase"])]
    enum Event {
        UserCreated {
            id: String,
        },
        #[arri(rename = "Removed")]
        UserDeleted(String),
    }

    // Checked on the raw output, as serde_json::Value doesn't preserve key order
    let serialized = Event::export().serialize().unwrap();
    let created = serialized
        .find("\"USER_CREATED\"")
        .expect("USER_CREATED tag");
    let removed = serialized.find("\"REMOVED\"").expect("REMOVED tag");
    assert!(created < removed, "variants should keep declaration order");
}

#[test]
fn test_internally_tagged_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(tag_key = "shape", transform = "lowercase")]
    enum Shape {
        Circle {
            radius: f64,
        },
        /// Nothing to see here
        Empty,
    }

    assert_eq!(
        schema_of::<Shape>(),
        json!({
            "tagKey": "shape",
            "union": {
                "circle": {
                    "properties": { "radius": { "type": "float64" } },
                    "optionalProperties": {}
                },
                "empty": {
                    "properties": {},
                    "optionalProperties": {},
                    "metadata": { "description": "Nothing to see here" }
                }
            },
            "metadata": { "id": "Shape" }
        })
    );
}

#[test]
fn test_adjacently_tagged_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(tag_key = "kind", value_key = "data")]
    enum Shape {
        Circle { radius: f64 },
        Label(String),
    }

    assert_eq!(
        schema_of::<Shape>(),
        json!({
            "tagKey": "kind",
            "valueKey": "data",
            "union": {
                "Circle": {
                    "properties": { "radius": { "type": "float64" } },
                    "optionalProperties": {}
                },
                "Label": { "type": "string" }
            },
            "metadata": { "id": "Shape" }
        })
    );
}

#[test]
fn test_recursive_union_export() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum Expr {
        Lit(i32),
        /// Negates an expression
        Neg(Box<Self>),
        Add {
            left: Box<Self>,
            right: Box<Self>,
        },
        Maybe(Option<Box<Self>>),
    }

    assert_eq!(
        schema_of::<Expr>(),
        json!({
            "union": {
                "Lit": { "type": "int32" },
                "Neg": { "ref": "Expr", "metadata": { "description": "Negates an expression" } },
                "Add": {
                    "properties": {
                        "left": { "ref": "Expr" },
                        "right": { "ref": "Expr" }
                    },
                    "optionalProperties": {}
                },
                "Maybe": { "ref": "Expr", "isNullable": true }
            },
            "metadata": { "id": "Expr" }
        })
    );
}

#[test]
fn test_nullable_union_field() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum Shape {
        Circle { radius: f64 },
    }

    #[allow(dead_code)]
    #[derive(Exported)]
    struct Drawing {
        #[arri(nullable)]
        shape: Option<Shape>,
    }

    let schema = schema_of::<Drawing>();
    assert_eq!(
        schema["optionalProperties"]["shape"]["isNullable"],
        json!(true)
    );
    assert!(schema["optionalProperties"]["shape"]["union"].is_object());
}

#[test]
fn test_result_export() {
    assert_eq!(
        schema_of::<Result<String, u32>>(),
        json!({
            "union": {
                "Ok": { "type": "string" },
                "Err": { "type": "uint32" }
            }
        })
    );

    assert_valid::<Result<String, u32>>(&Ok("yay".to_string()));
    assert_valid::<Result<String, u32>>(&Err(404));
}

#[test]
fn test_externally_tagged_matches_serde() {
    #[derive(Exported, Serialize, Deserialize)]
    enum Shape {
        Circle { radius: f64 },
        Label(String),
        Maybe(Option<u32>),
        Nested(Vec<Self>),
    }

    assert_valid(&Shape::Circle { radius: 2.5 });
    assert_valid(&Shape::Label("hi".to_string()));
    assert_valid(&Shape::Maybe(None));
    assert_valid(&Shape::Maybe(Some(4)));
    assert_valid(&Shape::Nested(vec![Shape::Circle { radius: 1.0 }]));
}

#[test]
fn test_internally_tagged_matches_serde() {
    #[derive(Exported, Serialize, Deserialize)]
    #[serde(tag = "shape", rename_all = "SCREAMING_SNAKE_CASE")]
    #[arri(tag_key = "shape", transform = ["snake_case", "uppercase"])]
    enum Shape {
        BigCircle {
            radius: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        },
        Empty,
    }

    assert_valid(&Shape::BigCircle {
        radius: 2.5,
        label: None,
    });
    assert_valid(&Shape::BigCircle {
        radius: 2.5,
        label: Some("big".to_string()),
    });
    assert_valid(&Shape::Empty);
}

#[test]
fn test_adjacently_tagged_matches_serde() {
    #[derive(Exported, Serialize, Deserialize)]
    #[serde(tag = "kind", content = "data")]
    #[arri(tag_key = "kind", value_key = "data")]
    enum Shape {
        Circle { radius: f64 },
        Label(String),
        Maybe(Option<u32>),
    }

    assert_valid(&Shape::Circle { radius: 2.5 });
    assert_valid(&Shape::Label("hi".to_string()));
    assert_valid(&Shape::Maybe(None));
}

#[test]
fn test_validator_rejects_mismatches() {
    // Sanity check that the validator above actually catches representation mismatches.
    #[derive(Exported, Serialize)]
    #[serde(tag = "kind")]
    enum InternallyTaggedInSerde {
        Circle { radius: f64 },
    }

    let schema = schema_of::<InternallyTaggedInSerde>();
    let json = serde_json::to_value(InternallyTaggedInSerde::Circle { radius: 1.0 }).unwrap();
    assert!(!validate(&schema, &schema, &json));
}

#[test]
fn test_unit_variants_in_unions() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum External {
        /// Nothing at all
        Unit,
        Braces {},
        Label(String),
    }

    #[allow(dead_code)]
    #[derive(Exported)]
    #[arri(tag_key = "kind", value_key = "data")]
    enum Adjacent {
        Unit,
        Label(String),
    }

    let empty = json!({ "properties": {}, "optionalProperties": {} });

    assert_eq!(
        schema_of::<External>(),
        json!({
            "union": {
                "Unit": {
                    "properties": {},
                    "optionalProperties": {},
                    "metadata": { "description": "Nothing at all" }
                },
                "Braces": empty,
                "Label": { "type": "string" }
            },
            "metadata": { "id": "External" }
        })
    );

    assert_eq!(
        schema_of::<Adjacent>(),
        json!({
            "tagKey": "kind",
            "valueKey": "data",
            "union": {
                "Unit": empty,
                "Label": { "type": "string" }
            },
            "metadata": { "id": "Adjacent" }
        })
    );
}

#[test]
fn test_unit_variants_match_serde_when_written_with_braces() {
    // serde writes `Unit {}` as an empty object, matching the convention for unit variants
    #[derive(Exported, Serialize, Deserialize)]
    enum External {
        Unit {},
        Label(String),
    }

    #[derive(Exported, Serialize, Deserialize)]
    #[serde(tag = "kind", content = "data")]
    #[arri(tag_key = "kind", value_key = "data")]
    enum Adjacent {
        Unit {},
        Label(String),
    }

    assert_valid(&External::Unit {});
    assert_valid(&External::Label("hi".to_string()));
    assert_valid(&Adjacent::Unit {});
    assert_valid(&Adjacent::Label("hi".to_string()));
}

#[test]
fn test_empty_variants_only_export_as_enum() {
    #[allow(dead_code)]
    #[derive(Exported)]
    enum Flags {
        A,
        B {},
        C(),
    }

    assert_eq!(
        schema_of::<Flags>(),
        json!({ "enum": ["A", "B", "C"], "metadata": { "id": "Flags" } })
    );
}
