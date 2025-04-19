use std::collections::HashMap;

use crate::{EnumTransformation, MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug, PartialEq, Eq)]
pub struct TaggedUnionSchema {
    pub discriminator: String,
    pub mapping: HashMap<String, Box<dyn Serializable>>,
    pub metadata: Option<MetadataSchema>,
    pub transformations: Vec<EnumTransformation>,
}

impl TaggedUnionSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_mapping(&mut self, variant: impl ToString, content: Box<dyn Serializable>) {
        let transformed = self
            .transformations
            .iter()
            .fold(variant.to_string(), |acc, transform| transform.apply(&acc));

        self.mapping.insert(transformed, content);
    }

    pub fn set_transforms(&mut self, transformations: &[EnumTransformation]) {
        self.transformations = transformations.into();
    }

    pub fn set_discriminator(&mut self, discriminator: impl ToString) {
        self.discriminator = discriminator.to_string();
    }
}

impl Default for TaggedUnionSchema {
    fn default() -> Self {
        Self {
            discriminator: "type".to_string(),
            mapping: HashMap::new(),
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
}
