use crate::{MetadataSchema, Serializable, serializer::Serializer};

/// Represents a schema for values in an Arri schema.
///
/// This struct defines the values, optional metadata, and nullability
/// associated with the schema.
#[derive(Debug)]
pub struct ValuesSchema {
    /// The values associated with the schema, represented as a serializable object.
    pub values: Box<dyn Serializable>,

    /// Optional metadata associated with the schema.
    pub metadata: Option<MetadataSchema>,

    /// Indicates whether the schema allows null values. If `Some(true)`,
    /// null values are permitted.
    pub is_nullable: Option<bool>,
}

impl ValuesSchema {
    /// Creates a new `ValuesSchema` instance with the specified values.
    ///
    /// # Arguments
    ///
    /// * `values` - A serializable object representing the values.
    ///
    /// # Returns
    ///
    /// A new `ValuesSchema` instance with default metadata and nullability.
    pub fn new(values: Box<dyn Serializable>) -> Self {
        Self {
            values,
            metadata: None,
            is_nullable: None,
        }
    }
}

impl Serializable for ValuesSchema {
    /// Serializes the `ValuesSchema` into a string representation.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the serialized schema, or `None` if serialization fails.
    fn serialize(&self) -> Option<String> {
        Serializer::builder()
            .set("values", &self.values)
            .set("metadata", &self.metadata)
            .set("isNullable", &self.is_nullable)
            .build()
            .into()
    }

    /// Sets the metadata for the schema.
    ///
    /// # Arguments
    ///
    /// * `metadata` - A `MetadataSchema` object to associate with the schema.
    fn set_metadata(&mut self, metadata: MetadataSchema) {
        self.metadata = Some(metadata);
    }

    /// Sets the nullability flag for the schema.
    ///
    /// # Arguments
    ///
    /// * `nullable` - A boolean indicating whether null values are allowed.
    fn set_nullable(&mut self, nullable: bool) {
        self.is_nullable = Some(nullable);
    }
}
