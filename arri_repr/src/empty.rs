use crate::{MetadataSchema, Serializable, serializer::Serializer};

/// Represents an empty schema in the Arri system.
///
/// This struct is used as a placeholder or default schema
/// when no specific schema is required. It includes optional
/// metadata for additional context.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct EmptySchema {
    /// Optional metadata associated with the schema.
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
