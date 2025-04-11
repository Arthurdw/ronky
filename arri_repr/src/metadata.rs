use crate::{Serializable, serializer::Serializer};

// TODO: docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataSchema {
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
}

impl MetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_id(&mut self, id: impl ToString) {
        self.id = Some(id.to_string());
    }

    pub fn set_description(&mut self, description: impl ToString) {
        self.description = Some(description.to_string());
    }

    pub fn set_is_deprecated(&mut self, is_deprecated: bool) {
        self.is_deprecated = Some(is_deprecated);
    }
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
