use crate::{Serializable, serializer::Serializer};

// TODO: docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataSchema {
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
}

// TODO: create a macro which automatically generates this implementation with a derive
impl Serializable for MetadataSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("id", &self.id)
            .set("description", &self.description)
            .set("isDeprecated", &self.is_deprecated)
            .build()
            .into()
    }
}
