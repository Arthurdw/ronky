use crate::{MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug, Eq)]
pub struct ElementSchema {
    pub elements: Box<dyn Serializable>,
    pub metadata: Option<MetadataSchema>,
    pub nullable: Option<bool>,
}

impl ElementSchema {
    pub fn new(elements: Box<dyn Serializable>) -> Self {
        Self {
            elements,
            metadata: None,
            nullable: None,
        }
    }
}

impl PartialEq for ElementSchema {
    fn eq(&self, other: &Self) -> bool {
        self.elements.eq(&other.elements)
    }
}

impl Serializable for ElementSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("elements", &self.elements)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }

    fn set_nullable(&mut self, nullable: bool) {
        self.nullable = Some(nullable);
    }
}

#[cfg(test)]
mod tests {
    use crate::{TypeSchema, Types};

    use super::*;

    #[test]
    fn test_vec_serialize() {
        let type_schema = ElementSchema::new(Box::new(TypeSchema::new(Types::String)));
        let serialized: serde_json::Value =
            serde_json::from_str(&type_schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({ "elements": { "type": "string" } })
        );
    }
}
