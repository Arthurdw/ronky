use crate::{EnumTransformation, MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct EnumSchema {
    pub r#enum: Vec<String>,
    pub metadata: Option<MetadataSchema>,
    pub transformations: Vec<EnumTransformation>,
}

impl EnumSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_variant(&mut self, variant: impl ToString) {
        let transformed = self
            .transformations
            .iter()
            .fold(variant.to_string(), |acc, transform| transform.apply(&acc));

        self.r#enum.push(transformed);
    }

    pub fn set_transforms(&mut self, transformations: &[EnumTransformation]) {
        self.transformations = transformations.into();
    }
}

impl Serializable for EnumSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("enum", &self.r#enum)
            .set("metadata", &self.metadata)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_serialize() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1");
        enum_schema.add_variant("Variant2");
        let serialized: serde_json::Value =
            serde_json::from_str(&enum_schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({ "enum": ["Variant1", "Variant2"] })
        );
    }

    #[test]
    fn test_add_variant() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        enum_schema.add_variant("Variant2".to_string());

        assert_eq!(
            enum_schema.r#enum,
            vec!["Variant1".to_string(), "Variant2".to_string()]
        );
    }

    #[test]
    fn test_set_metadata() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        let metadata = MetadataSchema::default();
        enum_schema.set_metadata(metadata.clone());

        assert_eq!(enum_schema.metadata, Some(metadata));
    }

    #[test]
    fn test_serialize_with_metadata() {
        let mut enum_schema = EnumSchema::new();
        enum_schema.add_variant("Variant1".to_string());
        let metadata = MetadataSchema::default();
        enum_schema.set_metadata(metadata);

        let serialized: serde_json::Value =
            serde_json::from_str(&enum_schema.serialize().unwrap()).unwrap();

        assert!(serialized.get("metadata").is_some());
    }
}
