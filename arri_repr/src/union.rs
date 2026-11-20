use indexmap::IndexMap;

use crate::{EnumTransformation, MetadataSchema, Serializable, serializer::Serializer};

/// Represents a schema for a union in an Arri schema (the "union" schema form).
///
/// A union describes a value that can be one of several variants. How the union is
/// represented in JSON depends on which keys are set:
///
/// | `tag_key` | `value_key` | Representation       | Example JSON                                  |
/// | --------- | ----------- | -------------------- | --------------------------------------------- |
/// | `None`    | `None`      | Externally tagged    | `{ "circle": { "radius": 2.5 } }`             |
/// | `Some`    | `None`      | Internally tagged    | `{ "shape": "circle", "radius": 2.5 }`        |
/// | `Some`    | `Some`      | Adjacently tagged    | `{ "kind": "circle", "data": { "radius": 2.5 } }` |
///
/// These line up with serde's default, `#[serde(tag = "...")]` and
/// `#[serde(tag = "...", content = "...")]` enum representations respectively.
///
/// Internally tagged unions may only use the "properties" schema form for their variants,
/// and those variants may not be nullable.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct UnionSchema {
    /// A mapping of variant tags to their corresponding schema.
    /// Uses IndexMap to preserve insertion order for deterministic serialization.
    pub union: IndexMap<String, Box<dyn Serializable>>,

    /// The JSON key holding the variant's tag. When `None` the union is externally tagged.
    pub tag_key: Option<String>,

    /// The JSON key holding the variant's value. Can only be used alongside `tag_key`.
    pub value_key: Option<String>,

    /// Optional metadata associated with the union schema.
    pub metadata: Option<MetadataSchema>,

    /// Indicates whether the union can be null.
    pub is_nullable: Option<bool>,

    /// A list of transformations applied to the variant tags.
    pub transformations: Vec<EnumTransformation>,
}

impl UnionSchema {
    /// Creates a new, externally tagged, `UnionSchema` without any variants.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a variant to the union.
    ///
    /// Any transformations set through [`UnionSchema::set_transforms`] are applied to the tag.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag identifying the variant.
    /// * `schema` - The schema of the variant.
    pub fn add_variant(&mut self, tag: impl ToString, schema: Box<dyn Serializable>) -> &mut Self {
        let transformed = self
            .transformations
            .iter()
            .fold(tag.to_string(), |acc, transform| transform.apply(&acc));

        self.union.insert(transformed, schema);
        self
    }

    /// Sets the transformations to be applied to variant tags.
    ///
    /// Only variants added after calling this are affected.
    pub fn set_transforms(&mut self, transformations: &[EnumTransformation]) -> &mut Self {
        self.transformations = transformations.into();
        self
    }

    /// Sets the key holding the variant's tag, making the union internally
    /// (or, together with a value key, adjacently) tagged.
    pub fn set_tag_key(&mut self, tag_key: impl ToString) -> &mut Self {
        self.tag_key = Some(tag_key.to_string());
        self
    }

    /// Sets the key holding the variant's value, making the union adjacently tagged.
    ///
    /// This must be used alongside [`UnionSchema::set_tag_key`].
    pub fn set_value_key(&mut self, value_key: impl ToString) -> &mut Self {
        self.value_key = Some(value_key.to_string());
        self
    }
}

impl Serializable for UnionSchema {
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("tagKey", &self.tag_key)
            .set("valueKey", &self.value_key)
            .set("union", &self.union)
            .set("metadata", &self.metadata)
            .set("isNullable", &self.is_nullable)
            .build()
            .into()
    }

    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(match self.metadata.take() {
            Some(current) => current | metadata,
            None => metadata,
        });
    }

    fn set_nullable(&mut self, is_nullable: bool) {
        self.is_nullable = Some(is_nullable);
    }
}

#[cfg(test)]
mod tests {
    use crate::{PropertiesSchema, TypeSchema, Types};

    use super::*;

    fn circle() -> Box<dyn Serializable> {
        let mut props = PropertiesSchema::new();
        props.set_property("radius", Box::new(TypeSchema::new(Types::Float64)));
        Box::new(props)
    }

    #[test]
    fn test_externally_tagged_serialize() {
        let mut schema = UnionSchema::new();
        schema.add_variant("circle", circle());
        schema.add_variant("label", Box::new(TypeSchema::new(Types::String)));
        schema.set_metadata(MetadataSchema::new().set_id("Shape").to_owned());

        let serialized: serde_json::Value =
            serde_json::from_str(&schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "union": {
                    "circle": {
                        "properties": { "radius": { "type": "float64" } },
                        "optionalProperties": {}
                    },
                    "label": { "type": "string" }
                },
                "metadata": { "id": "Shape" }
            })
        );
    }

    #[test]
    fn test_internally_tagged_serialize() {
        let mut schema = UnionSchema::new();
        schema.set_tag_key("shape");
        schema.add_variant("circle", circle());

        let serialized: serde_json::Value =
            serde_json::from_str(&schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "tagKey": "shape",
                "union": {
                    "circle": {
                        "properties": { "radius": { "type": "float64" } },
                        "optionalProperties": {}
                    }
                }
            })
        );
    }

    #[test]
    fn test_adjacently_tagged_serialize() {
        let mut schema = UnionSchema::new();
        schema.set_tag_key("kind").set_value_key("data");
        schema.add_variant("circle", circle());
        schema.set_nullable(true);

        let serialized: serde_json::Value =
            serde_json::from_str(&schema.serialize().unwrap()).unwrap();

        assert_eq!(
            serialized,
            serde_json::json!({
                "tagKey": "kind",
                "valueKey": "data",
                "union": {
                    "circle": {
                        "properties": { "radius": { "type": "float64" } },
                        "optionalProperties": {}
                    }
                },
                "isNullable": true
            })
        );
    }

    #[test]
    fn test_transforms_apply_to_tags() {
        let mut schema = UnionSchema::new();
        schema.set_transforms(&[EnumTransformation::Snakecase, EnumTransformation::Uppercase]);
        schema.add_variant("BigCircle", circle());

        assert!(schema.union.contains_key("BIG_CIRCLE"));
    }

    #[test]
    fn test_metadata_merges() {
        let mut schema = UnionSchema::new();
        schema.set_metadata(MetadataSchema::new().set_id("Shape").to_owned());
        schema.set_metadata(MetadataSchema::new().set_description("A shape").to_owned());

        let metadata = schema.metadata.unwrap();
        assert_eq!(metadata.id.as_deref(), Some("Shape"));
        assert_eq!(metadata.description.as_deref(), Some("A shape"));
    }

    #[test]
    fn test_preserves_variant_order() {
        let mut schema = UnionSchema::new();
        schema.add_variant("Zebra", circle());
        schema.add_variant("Apple", circle());
        schema.add_variant("Mango", circle());

        let serialized = schema.serialize().unwrap();
        let zebra = serialized.find("\"Zebra\"").unwrap();
        let apple = serialized.find("\"Apple\"").unwrap();
        let mango = serialized.find("\"Mango\"").unwrap();

        assert!(zebra < apple && apple < mango);
    }
}
