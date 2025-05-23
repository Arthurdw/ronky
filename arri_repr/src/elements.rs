use crate::{MetadataSchema, Serializable, serializer::Serializer};

/// Represents the schema for elements in arri
///
/// This struct defines the structure of elements, including their serializable
/// representation, optional metadata, and nullability.
///
/// # Fields
///
/// * `elements` - A boxed trait object implementing the `Serializable` trait,
///   representing the serializable content of the elements.
/// * `metadata` - An optional `MetadataSchema` providing additional information
///   about the elements.
/// * `is_nullable` - An optional boolean indicating whether the elements can
///   be null.
#[derive(Debug, Eq)]
pub struct ElementsSchema {
    pub elements: Box<dyn Serializable>,
    pub metadata: Option<MetadataSchema>,
    pub is_nullable: Option<bool>,
}

impl ElementsSchema {
    pub fn new(elements: Box<dyn Serializable>) -> Self {
        Self {
            elements,
            metadata: None,
            is_nullable: None,
        }
    }
}

impl PartialEq for ElementsSchema {
    fn eq(&self, other: &Self) -> bool {
        self.elements.eq(&other.elements)
    }
}

impl Serializable for ElementsSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("elements", &self.elements)
            .set("metadata", &self.metadata)
            .set("isNullable", &self.is_nullable)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }

    fn set_nullable(&mut self, nullable: bool) {
        self.is_nullable = Some(nullable);
    }
}

#[cfg(test)]
mod tests {
    use crate::{TypeSchema, Types};

    use super::*;

    #[test]
    fn test_vec_serialize() {
        let type_schema = ElementsSchema::new(Box::new(TypeSchema::new(Types::String)));
        let serialized: serde_json::Value =
            serde_json::from_str(&type_schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({ "elements": { "type": "string" } })
        );
    }

    #[test]
    fn test_vec_metadata() {
        let mut type_schema = ElementsSchema::new(Box::new(TypeSchema::new(Types::String)));
        type_schema.set_metadata(MetadataSchema::new().set_id("test").to_owned());

        assert_eq!(
            type_schema.metadata,
            Some(MetadataSchema::new().set_id("test").to_owned())
        );
    }

    #[test]
    fn test_vec_nullable() {
        let mut type_schema = ElementsSchema::new(Box::new(TypeSchema::new(Types::String)));
        type_schema.set_nullable(true);

        assert_eq!(type_schema.is_nullable, Some(true));
    }
}
