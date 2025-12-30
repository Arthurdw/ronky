use indexmap::IndexMap;

use crate::{MetadataSchema, Serializable, serializer::Serializer};

/// Represents a schema for properties in an Arri schema.
///
/// This struct defines the properties, optional properties, and metadata
/// associated with a schema. It also includes flags for strictness and nullability.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct PropertiesSchema {
    /// A map of required properties, where the key is the property name
    /// and the value is a serializable object. Uses IndexMap to preserve insertion order.
    pub properties: IndexMap<String, Box<dyn Serializable>>,

    /// A map of optional properties, where the key is the property name
    /// and the value is a serializable object. Uses IndexMap to preserve insertion order.
    pub optional_properties: IndexMap<String, Box<dyn Serializable>>,

    /// Indicates whether the schema is strict. If `Some(true)`, the schema
    /// enforces strict validation.
    pub is_strict: Option<bool>,

    /// Metadata associated with the schema.
    pub metadata: Option<MetadataSchema>,

    /// Indicates whether the schema allows null values. If `Some(true)`,
    /// null values are permitted.
    pub is_nullable: Option<bool>,
}

impl PropertiesSchema {
    /// Creates a new `PropertiesSchema` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or updates a required property in the schema.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the property.
    /// * `value` - A serializable object representing the property value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `PropertiesSchema` instance.
    pub fn set_property(&mut self, key: impl ToString, value: Box<dyn Serializable>) -> &mut Self {
        self.properties.insert(key.to_string(), value);
        self
    }

    /// Adds or updates an optional property in the schema.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the property.
    /// * `value` - A serializable object representing the property value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `PropertiesSchema` instance.
    pub fn set_optional_property(
        &mut self,
        key: impl ToString,
        value: Box<dyn Serializable>,
    ) -> &mut Self {
        self.optional_properties.insert(key.to_string(), value);
        self
    }

    /// Sets the strictness flag for the schema.
    ///
    /// # Arguments
    ///
    /// * `strict` - A boolean indicating whether the schema should enforce strict validation.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `PropertiesSchema` instance.
    pub fn set_strict(&mut self, strict: bool) -> &mut Self {
        self.is_strict = Some(strict);
        self
    }
}

impl Serializable for PropertiesSchema {
    /// Serializes the `PropertiesSchema` into a string representation.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the serialized schema, or `None` if serialization fails.
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("properties", &self.properties)
            .set("optionalProperties", &self.optional_properties)
            .set("isStrict", &self.is_strict)
            .set("metadata", &self.metadata)
            .set("isNullable", &self.is_nullable)
            .build()
            .into()
    }

    /// Sets the metadata for the schema.
    ///
    /// # Arguments
    ///
    /// * `metadata` - A `MetadataSchema` object to be merged with the existing metadata.
    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(if let Some(current) = &self.metadata {
            current.clone() | metadata
        } else {
            metadata
        });
    }

    /// Sets the nullability flag for the schema.
    ///
    /// # Arguments
    ///
    /// * `is_nullable` - A boolean indicating whether null values are allowed.
    fn set_nullable(&mut self, is_nullable: bool) {
        self.is_nullable = Some(is_nullable);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_schema_preserves_field_order() {
        let mut schema = PropertiesSchema::new();
        schema.set_property("aaa", Box::new("first"));
        schema.set_property("zzz", Box::new("second"));
        schema.set_property("mmm", Box::new("third"));

        let serialized = schema.serialize().unwrap();

        // Fields should appear in insertion order, not alphabetical
        let aaa_pos = serialized.find("\"aaa\"").unwrap();
        let zzz_pos = serialized.find("\"zzz\"").unwrap();
        let mmm_pos = serialized.find("\"mmm\"").unwrap();

        assert!(aaa_pos < zzz_pos, "aaa should come before zzz");
        assert!(zzz_pos < mmm_pos, "zzz should come before mmm");
    }

    #[test]
    fn test_optional_properties_schema_preserves_field_order() {
        let mut schema = PropertiesSchema::new();
        schema.set_optional_property("first_field", Box::new("a"));
        schema.set_optional_property("second_field", Box::new("b"));
        schema.set_optional_property("third_field", Box::new("c"));

        let serialized = schema.serialize().unwrap();

        // Fields should appear in insertion order
        let first_pos = serialized.find("\"first_field\"").unwrap();
        let second_pos = serialized.find("\"second_field\"").unwrap();
        let third_pos = serialized.find("\"third_field\"").unwrap();

        assert!(
            first_pos < second_pos,
            "first_field should come before second_field"
        );
        assert!(
            second_pos < third_pos,
            "second_field should come before third_field"
        );
    }
}
