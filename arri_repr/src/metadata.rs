use crate::{Serializable, serializer::Serializer};

// TODO: docs
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataSchema {
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
    pub deprecated_since: Option<String>,
    pub deprecated_message: Option<String>,
}

impl MetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_id(&mut self, id: impl ToString) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn set_description(&mut self, description: impl ToString) -> &mut Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn set_deprecated(&mut self, flag: bool) -> &mut Self {
        self.is_deprecated = Some(flag);
        self
    }

    pub fn set_deprecated_since(&mut self, version: impl ToString) -> &mut Self {
        self.deprecated_since = Some(version.to_string());
        self
    }

    pub fn set_deprecated_message(&mut self, message: impl ToString) -> &mut Self {
        self.deprecated_message = Some(message.to_string());
        self
    }
}

impl Serializable for MetadataSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("id", &self.id)
            .set("description", &self.description)
            .set("isDeprecated", &self.is_deprecated)
            .set("deprecatedSince", &self.deprecated_since)
            .set("deprecatedNote", &self.deprecated_message)
            .build()
            .into()
    }
}
