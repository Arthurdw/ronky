use ronky_derive::Serializable as SerializableDerive;

use crate::MetadataSchema;

/// Represents a reference schema in an Arri schema.
///
/// This struct is used to define a reference to another schema.
#[derive(Default, Debug, PartialEq, Eq, SerializableDerive)]
pub struct RefSchema {
    /// The reference string pointing to another schema.
    pub r#ref: String,

    /// Optional metadata associated with the reference.
    pub metadata: Option<MetadataSchema>,

    /// Indicates whether the reference allows null values.
    pub is_nullable: Option<bool>,
}

impl RefSchema {
    /// Creates a new `RefSchema` instance.
    ///
    /// # Arguments
    ///
    /// * `r#ref` - A value that can be converted to a string, representing the reference.
    ///
    /// # Returns
    ///
    /// A new `RefSchema` instance.
    pub fn new(r#ref: impl ToString) -> Self {
        Self {
            r#ref: r#ref.to_string(),
            ..Self::default()
        }
    }
}
