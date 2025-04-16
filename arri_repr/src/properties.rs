// TODO: docs
use std::collections::HashMap;

use crate::{MetadataSchema, Serializable, serializer::Serializer};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PropertiesSchema {
    pub properties: HashMap<String, Box<dyn Serializable>>,
    pub optional_properties: HashMap<String, Box<dyn Serializable>>,
    pub is_strict: Option<bool>,
    pub metadata: Option<MetadataSchema>,
}

impl PropertiesSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_property(&mut self, key: impl ToString, value: Box<dyn Serializable>) -> &mut Self {
        self.properties.insert(key.to_string(), value);
        self
    }

    pub fn set_optional_property(
        &mut self,
        key: impl ToString,
        value: Box<dyn Serializable>,
    ) -> &mut Self {
        self.optional_properties.insert(key.to_string(), value);
        self
    }

    pub fn set_strict(&mut self, strict: bool) -> &mut Self {
        self.is_strict = Some(strict);
        self
    }

    pub fn set_metadata(&mut self, metadata: MetadataSchema) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Serializable for PropertiesSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("properties", &self.properties)
            .set("optionalProperties", &self.optional_properties)
            .set("isStrict", &self.is_strict)
            .set("metadata", &self.metadata)
            .build()
            .into()
    }
}
