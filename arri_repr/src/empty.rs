use crate::{MetadataSchema, Serializable, serializer::Serializer};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct EmptySchema {
    metadata: Option<MetadataSchema>,
}

impl EmptySchema {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializable for EmptySchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("metadata", &self.metadata)
            .set("isNullable", &true)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }
}
