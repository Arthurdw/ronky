use std::ops::BitOr;

use crate::{Serializable, serializer::Serializer};

macro_rules! merge_fields {
    ($self:expr, $other:expr, $($field:ident),*) => {
        $(
            if let Some(value) = &$other.$field {
                $self.$field = Some(value.clone());
            }
        )*
    };
}

// TODO: docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataSchema {
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
    pub deprecated_since: Option<String>,
    pub deprecated_message: Option<String>,
}

impl MetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_id(&mut self, id: impl ToString) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn set_description(&mut self, description: impl ToString) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn set_deprecated(&mut self, flag: bool) -> &mut Self {
        self.is_deprecated = Some(flag);
        self
    }

    pub fn set_deprecated_since(&mut self, version: impl ToString) -> &mut Self {
        self.deprecated_since = Some(version.to_string());
        self
    }

    pub fn set_deprecated_message(&mut self, message: impl ToString) -> &mut Self {
        self.deprecated_message = Some(message.to_string());
        self
    }

    pub fn merge(mut self, other: &Self) -> Self {
        merge_fields!(
            self,
            other,
            id,
            description,
            deprecated_since,
            deprecated_message
        );

        // Handle non-String fields separately
        if let Some(is_deprecated) = other.is_deprecated {
            self.is_deprecated = Some(is_deprecated);
        }

        self
    }
}

impl Serializable for MetadataSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("id", &self.id)
            .set("description", &self.description)
            .set("isDeprecated", &self.is_deprecated)
            .set("deprecatedSince", &self.deprecated_since)
            .set("deprecatedNote", &self.deprecated_message)
            .build()
            .into()
    }
}

impl BitOr for MetadataSchema {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.merge(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_schema_defaults() {
        let schema = MetadataSchema::new();
        assert_eq!(schema.id, None);
        assert_eq!(schema.description, None);
        assert_eq!(schema.is_deprecated, None);
        assert_eq!(schema.deprecated_since, None);
        assert_eq!(schema.deprecated_message, None);
    }

    #[test]
    fn test_metadata_schema_setters() {
        let mut schema = MetadataSchema::new();
        schema
            .set_id("123")
            .set_description("Test description")
            .set_deprecated(true)
            .set_deprecated_since("1.0.0")
            .set_deprecated_message("Deprecated for testing");

        assert_eq!(schema.id, Some("123".to_string()));
        assert_eq!(schema.description, Some("Test description".to_string()));
        assert_eq!(schema.is_deprecated, Some(true));
        assert_eq!(schema.deprecated_since, Some("1.0.0".to_string()));
        assert_eq!(
            schema.deprecated_message,
            Some("Deprecated for testing".to_string())
        );
    }

    #[test]
    fn test_metadata_schema_merge() {
        let mut schema1 = MetadataSchema::new();
        schema1.set_id("123").set_description("Schema 1");

        let mut schema2 = MetadataSchema::new();
        schema2
            .set_description("Schema 2")
            .set_deprecated(true)
            .set_deprecated_since("2.0.0");

        let merged = schema1.merge(&schema2);

        assert_eq!(merged.id, Some("123".to_string()));
        assert_eq!(merged.description, Some("Schema 2".to_string()));
        assert_eq!(merged.is_deprecated, Some(true));
        assert_eq!(merged.deprecated_since, Some("2.0.0".to_string()));
        assert_eq!(merged.deprecated_message, None);
    }

    #[test]
    fn test_metadata_schema_serialization() {
        let mut schema = MetadataSchema::new();
        schema
            .set_id("123")
            .set_description("Test description")
            .set_deprecated(true)
            .set_deprecated_since("1.0.0")
            .set_deprecated_message("Deprecated for testing");

        let serialized = schema.serialize();
        assert!(serialized.is_some());
        let serialized_str = serialized.unwrap();
        assert!(serialized_str.contains("\"id\":\"123\""));
        assert!(serialized_str.contains("\"description\":\"Test description\""));
        assert!(serialized_str.contains("\"isDeprecated\":true"));
        assert!(serialized_str.contains("\"deprecatedSince\":\"1.0.0\""));
        assert!(serialized_str.contains("\"deprecatedNote\":\"Deprecated for testing\""));
    }

    #[test]
    fn test_metadata_schema_bitor_operator() {
        let mut schema1 = MetadataSchema::new();
        schema1.set_id("123");

        let mut schema2 = MetadataSchema::new();
        schema2.set_description("Schema 2");

        let merged = schema1 | schema2;

        assert_eq!(merged.id, Some("123".to_string()));
        assert_eq!(merged.description, Some("Schema 2".to_string()));
    }
}
