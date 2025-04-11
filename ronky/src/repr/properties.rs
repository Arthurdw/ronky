use std::collections::HashMap;

use crate::serialization::Serializable;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PropertiesSchema {
    pub properties: HashMap<String, Box<dyn Serializable>>,
    pub optional_properties: HashMap<String, Box<dyn Serializable>>,
    pub strict: Option<bool>,
}
