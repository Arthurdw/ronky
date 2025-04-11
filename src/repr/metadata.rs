#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct MetadataSchema {
    pub id: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
}
