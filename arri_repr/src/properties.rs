// TODO: docs
use std::collections::HashMap;

use crate::{Serializable, serializer::Serializer};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PropertiesSchema {
    pub properties: HashMap<String, Box<dyn Serializable>>,
    pub optional_properties: HashMap<String, Box<dyn Serializable>>,
    pub strict: Option<bool>,
}

impl Serializable for PropertiesSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            // .set("properties", &self.properties)
            // .set("optionalProperties", &self.optional_properties)
            .set("strict", &self.strict)
            .build()
            .into()
    }
}
