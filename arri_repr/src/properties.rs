// TODO: docs
use std::collections::HashMap;

use crate::{Serializable, serializer::Serializer};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PropertiesSchema {
    pub properties: HashMap<String, Box<dyn Serializable>>,
    pub optional_properties: HashMap<String, Box<dyn Serializable>>,
    pub strict: Option<bool>,
    pub metadata: Option<Box<dyn Serializable>>,
}

impl PropertiesSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_property(&mut self, key: String, value: Box<dyn Serializable>) {
        self.properties.insert(key, value);
    }

    pub fn set_optional_property(&mut self, key: String, value: Box<dyn Serializable>) {
        self.optional_properties.insert(key, value);
    }

    pub fn set_strict(&mut self, strict: bool) {
        self.strict = Some(strict);
    }

    pub fn set_metadata(&mut self, metadata: Box<dyn Serializable>) {
        self.metadata = Some(metadata);
    }
}

impl Serializable for PropertiesSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("properties", &self.properties)
            .set("optionalProperties", &self.optional_properties)
            .set("strict", &self.strict)
            .set("metadata", &self.metadata)
            .build()
            .into()
    }
}
