use indexmap::IndexMap;

use crate::{EnumTransformation, MetadataSchema, Serializable, serializer::Serializer};
/// Represents a schema for a tagged union in an Arri schema.
///
/// This struct defines the discriminator, mapping of variants, optional metadata,
/// and transformations associated with a tagged union.
#[derive(Debug, PartialEq, Eq)]
pub struct TaggedUnionSchema {
    /// The discriminator used to identify the variant in the tagged union.
    pub discriminator: String,

    /// A mapping of variant names to their corresponding serializable content.
    /// Uses IndexMap to preserve insertion order for deterministic serialization.
    pub mapping: IndexMap<String, Box<dyn Serializable>>,

    /// Optional metadata associated with the tagged union schema.
    pub metadata: Option<MetadataSchema>,

    /// A list of transformations applied to the variant names.
    pub transformations: Vec<EnumTransformation>,
}

impl TaggedUnionSchema {
    /// Creates a new `TaggedUnionSchema` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `TaggedUnionSchema` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a mapping for a variant in the tagged union.
    ///
    /// # Arguments
    ///
    /// * `variant` - The name of the variant.
    /// * `content` - The serializable content associated with the variant.
    pub fn add_mapping(&mut self, variant: impl ToString, content: Box<dyn Serializable>) {
        let transformed = self
            .transformations
            .iter()
            .fold(variant.to_string(), |acc, transform| transform.apply(&acc));

        self.mapping.insert(transformed, content);
    }

    /// Sets the transformations to be applied to variant names.
    ///
    /// # Arguments
    ///
    /// * `transformations` - A slice of `EnumTransformation` objects.
    pub fn set_transforms(&mut self, transformations: &[EnumTransformation]) {
        self.transformations = transformations.into();
    }

    /// Sets the discriminator for the tagged union.
    ///
    /// # Arguments
    ///
    /// * `discriminator` - The discriminator value as a string.
    pub fn set_discriminator(&mut self, discriminator: impl ToString) {
        self.discriminator = discriminator.to_string();
    }
}

impl Default for TaggedUnionSchema {
    fn default() -> Self {
        Self {
            discriminator: "type".to_string(),
            mapping: IndexMap::new(),
            metadata: None,
            transformations: Vec::new(),
        }
    }
}

impl Serializable for TaggedUnionSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("discriminator", &self.discriminator)
            .set("mapping", &self.mapping)
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
    use crate::{TypeSchema, Types};

    use super::*;

    #[test]
    fn test_tagged_union_serialize() {
        let mut tagged = TaggedUnionSchema::new();
        tagged.set_discriminator("type");
        tagged.add_mapping("Variant1", Box::new(TypeSchema::new(Types::String)));

        tagged.set_metadata(
            MetadataSchema::new()
                .set_id("ExampleEnum")
                .set_description("An example tagged union")
                .to_owned(),
        );

        let serialized: serde_json::Value =
            serde_json::from_str(&tagged.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "discriminator": "type",
                "mapping": {
                    "Variant1": {
                        "type": "string"
                    }
                },
                "metadata": {
                    "id": "ExampleEnum",
                    "description": "An example tagged union"
                }
            })
        );
    }

    #[test]
    fn test_tagged_union_preserves_variant_order() {
        let mut schema = TaggedUnionSchema::new();
        schema.set_discriminator("type");

        // Add variants in specific order (not alphabetical)
        schema.add_mapping("Zebra", Box::new(TypeSchema::new(Types::String)));
        schema.add_mapping("Apple", Box::new(TypeSchema::new(Types::String)));
        schema.add_mapping("Mango", Box::new(TypeSchema::new(Types::String)));

        let serialized = schema.serialize().unwrap();

        // Variants should appear in insertion order, not alphabetical
        let zebra_pos = serialized.find("\"Zebra\"").unwrap();
        let apple_pos = serialized.find("\"Apple\"").unwrap();
        let mango_pos = serialized.find("\"Mango\"").unwrap();

        assert!(zebra_pos < apple_pos, "Zebra should come before Apple");
        assert!(apple_pos < mango_pos, "Apple should come before Mango");
    }
}
