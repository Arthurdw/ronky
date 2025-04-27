use crate::{MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug)]
pub struct ValuesSchema {
    pub values: Box<dyn Serializable>,
    pub metadata: Option<MetadataSchema>,
    pub is_nullable: Option<bool>,
}

impl ValuesSchema {
    pub fn new(values: Box<dyn Serializable>) -> Self {
        Self {
            values,
            metadata: None,
            is_nullable: None,
        }
    }
}

impl Serializable for ValuesSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("values", &self.values)
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
